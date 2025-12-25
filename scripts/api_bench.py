#!/usr/bin/env python3
"""
Robust HTTP Benchmark Tool
Author: Maxime-Cllt
Date: 2025-07-03
"""

import asyncio
import aiohttp
import time
import statistics
import sys
import argparse
from dataclasses import dataclass, field
from typing import List, Dict, Optional
from collections import defaultdict
import json


@dataclass
class BenchmarkConfig:
    """Configuration for the benchmark"""
    url: str = "http://localhost:8080/api/get-speed/today"
    num_requests: int = 50000
    concurrency: int = 100
    timeout: float = 30.0
    warmup_requests: int = 100
    output_format: str = "console"  # console, json, csv
    headers: Dict[str, str] = field(default_factory=dict)
    method: str = "GET"
    body: Optional[str] = None


@dataclass
class BenchmarkResults:
    """Results container with optimized storage"""
    success_count: int = 0
    failure_count: int = 0
    timeout_count: int = 0
    total_time: float = 0.0
    response_times: List[float] = field(default_factory=list)
    status_codes: Dict[int, int] = field(default_factory=lambda: defaultdict(int))
    error_types: Dict[str, int] = field(default_factory=lambda: defaultdict(int))
    warmup_time: float = 0.0


class HighPrecisionTimer:
    """High precision timer using time.perf_counter()"""

    @staticmethod
    def now() -> float:
        return time.perf_counter()

    @staticmethod
    def to_milliseconds(seconds: float) -> float:
        return seconds * 1000

    @staticmethod
    def to_microseconds(seconds: float) -> float:
        return seconds * 1_000_000

    @staticmethod
    def to_nanoseconds(seconds: float) -> float:
        return seconds * 1_000_000_000


class RobustBenchmark:
    """Optimized benchmark runner"""

    def __init__(self, config: BenchmarkConfig):
        self.config = config
        self.results = BenchmarkResults()
        self.timer = HighPrecisionTimer()

        # Pre-allocate response times list for better performance
        self.results.response_times = [0.0] * config.num_requests
        self._response_index = 0

    async def _make_request(self, session: aiohttp.ClientSession) -> None:
        """Optimized request function"""
        start_time = self.timer.now()

        try:
            timeout = aiohttp.ClientTimeout(total=self.config.timeout)

            if self.config.method.upper() == "GET":
                async with session.get(
                        self.config.url,
                        timeout=timeout,
                        headers=self.config.headers
                ) as response:
                    # Read response body to ensure complete transfer
                    await response.read()
                    elapsed = self.timer.now() - start_time

                    # Thread-safe operations
                    if self._response_index < len(self.results.response_times):
                        self.results.response_times[self._response_index] = elapsed
                        self._response_index += 1

                    self.results.status_codes[response.status] += 1

                    if 200 <= response.status < 300:
                        self.results.success_count += 1
                    else:
                        self.results.failure_count += 1

            elif self.config.method.upper() == "POST":
                async with session.post(
                        self.config.url,
                        data=self.config.body,
                        timeout=timeout,
                        headers=self.config.headers
                ) as response:
                    await response.read()
                    elapsed = self.timer.now() - start_time

                    if self._response_index < len(self.results.response_times):
                        self.results.response_times[self._response_index] = elapsed
                        self._response_index += 1

                    self.results.status_codes[response.status] += 1

                    if 200 <= response.status < 300:
                        self.results.success_count += 1
                    else:
                        self.results.failure_count += 1

        except asyncio.TimeoutError:
            self.results.timeout_count += 1
            self.results.failure_count += 1
            self.results.error_types["timeout"] += 1
        except aiohttp.ClientError as e:
            self.results.failure_count += 1
            self.results.error_types[type(e).__name__] += 1
        except Exception as e:
            self.results.failure_count += 1
            self.results.error_types[type(e).__name__] += 1

    async def _bound_request(self, semaphore: asyncio.Semaphore, session: aiohttp.ClientSession) -> None:
        """Semaphore-controlled request"""
        async with semaphore:
            await self._make_request(session)

    async def _warmup(self, session: aiohttp.ClientSession) -> None:
        """Warmup phase to stabilize performance"""
        print(f"🔥 Warming up with {self.config.warmup_requests} requests...")

        warmup_start = self.timer.now()
        semaphore = asyncio.Semaphore(min(self.config.concurrency, self.config.warmup_requests))

        tasks = [
            self._bound_request(semaphore, session)
            for _ in range(self.config.warmup_requests)
        ]

        await asyncio.gather(*tasks, return_exceptions=True)
        self.results.warmup_time = self.timer.now() - warmup_start

        # Reset counters after warmup
        self.results.success_count = 0
        self.results.failure_count = 0
        self.results.timeout_count = 0
        self.results.status_codes.clear()
        self.results.error_types.clear()
        self._response_index = 0

        print(f"✅ Warmup completed in {self.results.warmup_time:.3f}s")

    async def run(self) -> BenchmarkResults:
        """Run the benchmark"""
        print(f"🚀 Starting benchmark: {self.config.num_requests} requests, {self.config.concurrency} concurrent")
        print(f"🎯 Target: {self.config.url}")

        # Optimized connector settings
        connector = aiohttp.TCPConnector(
            limit=self.config.concurrency * 2,
            limit_per_host=self.config.concurrency,
            ttl_dns_cache=300,
            use_dns_cache=True,
            keepalive_timeout=30,
            enable_cleanup_closed=True
        )

        timeout = aiohttp.ClientTimeout(total=self.config.timeout)

        async with aiohttp.ClientSession(
                connector=connector,
                timeout=timeout,
                headers=self.config.headers
        ) as session:
            # Warmup phase
            if self.config.warmup_requests > 0:
                await self._warmup(session)

            # Main benchmark
            print(f"⚡ Running main benchmark...")
            semaphore = asyncio.Semaphore(self.config.concurrency)

            start_time = self.timer.now()

            tasks = [
                self._bound_request(semaphore, session)
                for _ in range(self.config.num_requests)
            ]

            await asyncio.gather(*tasks, return_exceptions=True)

            self.results.total_time = self.timer.now() - start_time

        # Trim unused response times
        self.results.response_times = self.results.response_times[:self._response_index]

        return self.results

    def generate_report(self) -> Dict:
        """Generate comprehensive benchmark report"""
        times = [t for t in self.results.response_times if t > 0]

        if not times:
            return {"error": "No successful requests recorded"}

        # Calculate statistics
        times_ms = [self.timer.to_milliseconds(t) for t in times]
        times_us = [self.timer.to_microseconds(t) for t in times]
        times_ns = [self.timer.to_nanoseconds(t) for t in times]

        # Percentiles
        percentiles = [50, 75, 90, 95, 99, 99.9]
        percentile_data = {
            f"p{p}": {
                "seconds": statistics.quantiles(times, n=1000)[int(p*10)-1] if p < 99.9 else max(times),
                "milliseconds": statistics.quantiles(times_ms, n=1000)[int(p*10)-1] if p < 99.9 else max(times_ms),
                "microseconds": statistics.quantiles(times_us, n=1000)[int(p*10)-1] if p < 99.9 else max(times_us),
            }
            for p in percentiles
        }

        # Success rate
        total_requests = self.results.success_count + self.results.failure_count
        success_rate = (self.results.success_count / total_requests * 100) if total_requests > 0 else 0

        # Throughput
        throughput = self.results.success_count / self.results.total_time if self.results.total_time > 0 else 0

        return {
            "summary": {
                "total_requests": total_requests,
                "successful_requests": self.results.success_count,
                "failed_requests": self.results.failure_count,
                "timeout_requests": self.results.timeout_count,
                "success_rate_percent": round(success_rate, 2),
                "total_duration_seconds": round(self.results.total_time, 4),
                "warmup_duration_seconds": round(self.results.warmup_time, 4),
                "throughput_req_per_sec": round(throughput, 2),
            },
            "response_times": {
                "average": {
                    "seconds": round(statistics.mean(times), 6),
                    "milliseconds": round(statistics.mean(times_ms), 3),
                    "microseconds": round(statistics.mean(times_us), 1),
                    "nanoseconds": round(statistics.mean(times_ns), 0),
                },
                "median": {
                    "seconds": round(statistics.median(times), 6),
                    "milliseconds": round(statistics.median(times_ms), 3),
                    "microseconds": round(statistics.median(times_us), 1),
                },
                "min": {
                    "seconds": round(min(times), 6),
                    "milliseconds": round(min(times_ms), 3),
                    "microseconds": round(min(times_us), 1),
                },
                "max": {
                    "seconds": round(max(times), 6),
                    "milliseconds": round(max(times_ms), 3),
                    "microseconds": round(max(times_us), 1),
                },
                "std_deviation": {
                    "seconds": round(statistics.stdev(times), 6),
                    "milliseconds": round(statistics.stdev(times_ms), 3),
                },
                "percentiles": percentile_data,
            },
            "status_codes": dict(self.results.status_codes),
            "error_types": dict(self.results.error_types),
            "config": {
                "url": self.config.url,
                "method": self.config.method,
                "concurrency": self.config.concurrency,
                "timeout_seconds": self.config.timeout,
                "warmup_requests": self.config.warmup_requests,
            }
        }

    def print_report(self, report: Dict) -> None:
        """Print formatted console report"""
        print("\n" + "="*80)
        print("📊 BENCHMARK RESULTS")
        print("="*80)

        summary = report["summary"]
        response_times = report["response_times"]

        print(f"\n📈 SUMMARY:")
        print(f"  Total Requests:     {summary['total_requests']:,}")
        print(f"  ✅ Successful:      {summary['successful_requests']:,} ({summary['success_rate_percent']}%)")
        print(f"  ❌ Failed:          {summary['failed_requests']:,}")
        print(f"  ⏰ Timeouts:        {summary['timeout_requests']:,}")
        print(f"  🕐 Total Duration:   {summary['total_duration_seconds']:.4f}s")
        print(f"  🔥 Warmup Duration:  {summary['warmup_duration_seconds']:.4f}s")
        print(f"  ⚡ Throughput:       {summary['throughput_req_per_sec']:,.2f} req/sec")

        print(f"\n⏱️  RESPONSE TIMES:")
        avg = response_times["average"]
        print(f"  Average:  {avg['milliseconds']:8.3f} ms  ({avg['microseconds']:8.1f} μs)")

        median = response_times["median"]
        print(f"  Median:   {median['milliseconds']:8.3f} ms  ({median['microseconds']:8.1f} μs)")

        min_time = response_times["min"]
        print(f"  Minimum:  {min_time['milliseconds']:8.3f} ms  ({min_time['microseconds']:8.1f} μs)")

        max_time = response_times["max"]
        print(f"  Maximum:  {max_time['milliseconds']:8.3f} ms  ({max_time['microseconds']:8.1f} μs)")

        std = response_times["std_deviation"]
        print(f"  Std Dev:  {std['milliseconds']:8.3f} ms")

        print(f"\n📊 PERCENTILES:")
        for percentile, data in response_times["percentiles"].items():
            print(f"  {percentile:>4}: {data['milliseconds']:8.3f} ms  ({data['microseconds']:8.1f} μs)")

        if report["status_codes"]:
            print(f"\n🔢 STATUS CODES:")
            for code, count in sorted(report["status_codes"].items()):
                print(f"  {code}: {count:,}")

        if report["error_types"]:
            print(f"\n❌ ERROR TYPES:")
            for error_type, count in sorted(report["error_types"].items()):
                print(f"  {error_type}: {count:,}")

        print("\n" + "="*80)


def main():
    parser = argparse.ArgumentParser(description="Robust HTTP Benchmark Tool")
    parser.add_argument("--url", default="http://localhost:8080/health", help="Target URL")
    parser.add_argument("--requests", "-n", type=int, default=50000, help="Number of requests")
    parser.add_argument("--concurrency", "-c", type=int, default=100, help="Concurrent requests")
    parser.add_argument("--timeout", "-t", type=float, default=30.0, help="Request timeout in seconds")
    parser.add_argument("--warmup", type=int, default=100, help="Warmup requests")
    parser.add_argument("--method", default="GET", choices=["GET", "POST"], help="HTTP method")
    parser.add_argument("--output", choices=["console", "json"], default="console", help="Output format")
    parser.add_argument("--body", help="Request body for POST requests")
    parser.add_argument("--header", action="append", help="HTTP header (format: 'Key: Value')")

    args = parser.parse_args()

    # Parse headers
    headers = {}
    if args.header:
        for header in args.header:
            if ": " in header:
                key, value = header.split(": ", 1)
                headers[key] = value

    config = BenchmarkConfig(
        url=args.url,
        num_requests=args.requests,
        concurrency=args.concurrency,
        timeout=args.timeout,
        warmup_requests=args.warmup,
        method=args.method,
        output_format=args.output,
        headers=headers,
        body=args.body
    )

    benchmark = RobustBenchmark(config)

    try:
        results = asyncio.run(benchmark.run())
        report = benchmark.generate_report()

        if args.output == "json":
            print(json.dumps(report, indent=2))
        else:
            benchmark.print_report(report)

    except KeyboardInterrupt:
        print("\n⚠️  Benchmark interrupted by user")
        sys.exit(1)
    except Exception as e:
        print(f"\n❌ Benchmark failed: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()