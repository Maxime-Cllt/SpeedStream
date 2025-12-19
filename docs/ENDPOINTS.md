# SpeedStream API Documentation

## Base URL
```
http://localhost:8080
```

## Authentication

All API endpoints (except `/` and `/health`) require Bearer token authentication.

**Authentication Header Format**
```
Authorization: Bearer <your-token>
```

**Example**
```bash
curl -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds/latest
```

**Status Codes**
- `401 Unauthorized` - Missing or invalid token
- `500 Internal Server Error` - Authentication service error

**Token Validation**
- Tokens are validated against the database
- Valid tokens are cached in Redis for improved performance
- Tokens must be active and not expired

## Table of Contents
- [Health Check](#health-check)
- [Speed Measurements](#speed-measurements)
  - [Create Speed Measurement](#create-speed-measurement)
  - [Get Speed Measurements](#get-speed-measurements)
  - [Get Latest Speed](#get-latest-speed)
  - [Get Today's Speeds](#get-todays-speeds)
  - [Get Paginated Speeds](#get-paginated-speeds)
  - [Get Speeds by Date Range](#get-speeds-by-date-range)
  - [Real-time Speed Stream (SSE)](#real-time-speed-stream-sse)

---

## Health Check

### `GET /health`

Check if the API and database are healthy.

**Response**
```json
{
  "status": "ok",
  "message": "API is healthy! Current time: 2025-11-25 14:30:00"
}
```

**Status Codes**
- `200 OK` - Service is healthy
- `503 Service Unavailable` - Database connection failed

---

## Speed Measurements

### Create Speed Measurement

**`POST /api/speeds`**

Create a new speed measurement from a sensor.

🔒 **Requires Authentication**: This endpoint requires a valid Bearer token in the Authorization header.

**Request Body**
```json
{
  "sensor_name": "Sensor A",  // Optional: Name of the sensor
  "speed": 65.5,              // Required: Speed in km/h (float)
  "lane": 0                   // Required: Lane identifier (0=Left, 1=Right)
}
```

**Fields**
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sensor_name` | string | No | Name/identifier of the sensor |
| `speed` | float | Yes | Speed measurement in km/h |
| `lane` | integer | Yes | Lane identifier: `0` (Left) or `1` (Right) |

**Example Request**
```bash
curl -X POST http://localhost:8080/api/speeds \
  -H "Authorization: Bearer your_api_token_here" \
  -H "Content-Type: application/json" \
  -d '{
    "sensor_name": "Highway Sensor 001",
    "speed": 75.3,
    "lane": 1
  }'
```

**Response**
- `201 Created` - Speed measurement successfully created
- `400 Bad Request` - Invalid request payload
- `500 Internal Server Error` - Database error

**Notes**
- The timestamp (`created_at`) is automatically set by the database
- This endpoint updates the Redis cache with the latest measurement for performance optimization

---

### Get Speed Measurements

**`GET /api/speeds?limit={n}`**

Retrieve the last N speed measurements from the database.

🔒 **Requires Authentication**: This endpoint requires a valid Bearer token in the Authorization header.

**Query Parameters**
| Parameter | Type | Default | Max | Description |
|-----------|------|---------|-----|-------------|
| `limit` | integer | 100 | 1000 | Number of records to retrieve |

**Example Request**
```bash
curl -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds?limit=50
```

**Response**
```json
[
  {
    "id": 123,
    "sensor_name": "Sensor A",
    "speed": 75.3,
    "lane": 1,
    "created_at": "2025-11-25T14:30:00.123456Z"
  },
  {
    "id": 122,
    "sensor_name": null,
    "speed": 62.1,
    "lane": 0,
    "created_at": "2025-11-25T14:29:45.789012Z"
  }
]
```

**Response Fields**
| Field | Type | Description |
|-------|------|-------------|
| `id` | integer | Unique identifier for the measurement |
| `sensor_name` | string or null | Name of the sensor (if provided) |
| `speed` | float | Speed in km/h |
| `lane` | integer | Lane identifier: `0` (Left) or `1` (Right) |
| `created_at` | ISO 8601 datetime | Timestamp when the measurement was recorded |

**Status Codes**
- `200 OK` - Success
- `500 Internal Server Error` - Database error

---

### Get Latest Speed

**`GET /api/speeds/latest`**

Retrieve the most recent speed measurement. This endpoint uses Redis caching for optimal performance.

🔒 **Requires Authentication**: This endpoint requires a valid Bearer token in the Authorization header.

**Example Request**
```bash
curl -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds/latest
```

**Response**
```json
{
  "id": 123,
  "sensor_name": "Highway Sensor 001",
  "speed": 75.3,
  "lane": 1,
  "created_at": "2025-11-25T14:30:00.123456Z"
}
```

**Status Codes**
- `200 OK` - Success
- `500 Internal Server Error` - Database error or no data available

**Performance Notes**
- First request: Fetches from database and caches in Redis (TTL: 1 hour)
- Subsequent requests: Served from Redis cache (significantly faster)
- Cache is automatically updated when new measurements are created via `POST /api/speeds`

---

### Get Today's Speeds

**`GET /api/speeds/today?limit={n}`**

Retrieve all speed measurements recorded today (from midnight UTC).

🔒 **Requires Authentication**: This endpoint requires a valid Bearer token in the Authorization header.

**Query Parameters**
| Parameter | Type | Default | Max | Description |
|-----------|------|---------|-----|-------------|
| `limit` | integer | 100 | 1000 | Maximum number of records to retrieve |

**Example Request**
```bash
curl -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds/today?limit=200
```

**Response**
```json
[
  {
    "id": 123,
    "sensor_name": "Sensor A",
    "speed": 75.3,
    "lane": 1,
    "created_at": "2025-11-25T14:30:00.123456Z"
  },
  {
    "id": 122,
    "sensor_name": "Sensor B",
    "speed": 62.1,
    "lane": 0,
    "created_at": "2025-11-25T12:15:30.456789Z"
  }
]
```

**Status Codes**
- `200 OK` - Success (may return empty array if no data today)
- `500 Internal Server Error` - Database error

---

### Get Paginated Speeds

**`GET /api/speeds/paginated?offset={n}&limit={m}`**

Retrieve speed measurements with pagination support for efficient data browsing.

🔒 **Requires Authentication**: This endpoint requires a valid Bearer token in the Authorization header.

**Query Parameters**
| Parameter | Type | Default | Max | Description |
|-----------|------|---------|-----|-------------|
| `offset` | integer | 0 | - | Number of records to skip |
| `limit` | integer | 100 | 1000 | Number of records to retrieve |

**Example Request**
```bash
# Get records 100-149 (page 2 with 50 items per page)
curl -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds/paginated?offset=100&limit=50
```

**Response**
```json
[
  {
    "id": 100,
    "sensor_name": "Sensor C",
    "speed": 68.7,
    "lane": 0,
    "created_at": "2025-11-25T10:45:22.987654Z"
  }
]
```

**Status Codes**
- `200 OK` - Success
- `500 Internal Server Error` - Database error

**Pagination Example**
```javascript
// JavaScript example for pagination
const itemsPerPage = 50;
const currentPage = 1;
const offset = currentPage * itemsPerPage;

fetch(`http://localhost:8080/api/speeds/paginated?offset=${offset}&limit=${itemsPerPage}`, {
  headers: {
    'Authorization': 'Bearer your_api_token_here'
  }
})
  .then(response => response.json())
  .then(data => console.log(data));
```

---

### Get Speeds by Date Range

**`GET /api/speeds/range?start_date={start}&end_date={end}`**

Retrieve all speed measurements recorded within a specific date range. This endpoint is useful for generating reports, analyzing historical data, or exporting data for a specific time period.

🔒 **Requires Authentication**: This endpoint requires a valid Bearer token in the Authorization header.

**Query Parameters**
| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `start_date` | string | Yes | Start of the date range (inclusive) |
| `end_date` | string | Yes | End of the date range (inclusive) |

**Date Format**
The API accepts dates in two formats:
- **Date only**: `YYYY-MM-DD` (e.g., `2024-01-15`)
- **Date with time**: `YYYY-MM-DD HH:MM:SS` (e.g., `2024-01-15 14:30:00`)

**Example Requests**

```bash
# Get all speeds for a specific day
curl -H "Authorization: Bearer your_api_token_here" \
  "http://localhost:8080/api/speeds/range?start_date=2024-01-15&end_date=2024-01-15"

# Get all speeds for a month
curl -H "Authorization: Bearer your_api_token_here" \
  "http://localhost:8080/api/speeds/range?start_date=2024-01-01&end_date=2024-01-31"

# Get speeds with specific time range
curl -H "Authorization: Bearer your_api_token_here" \
  "http://localhost:8080/api/speeds/range?start_date=2024-01-15%2008:00:00&end_date=2024-01-15%2018:00:00"

# URL encoded version (spaces become %20)
curl -H "Authorization: Bearer your_api_token_here" \
  "http://localhost:8080/api/speeds/range?start_date=2024-01-15+08:00:00&end_date=2024-01-15+18:00:00"
```

**Response**
```json
[
  {
    "id": 456,
    "sensor_name": "Highway Sensor 001",
    "speed": 75.3,
    "lane": 1,
    "created_at": "2024-01-15T08:15:23.123456Z"
  },
  {
    "id": 457,
    "sensor_name": "Highway Sensor 002",
    "speed": 68.5,
    "lane": 0,
    "created_at": "2024-01-15T09:22:10.987654Z"
  },
  {
    "id": 458,
    "sensor_name": null,
    "speed": 82.1,
    "lane": 1,
    "created_at": "2024-01-15T14:45:55.456789Z"
  }
]
```

**Status Codes**
- `200 OK` - Success (may return empty array if no data in range)
- `400 Bad Request` - Invalid date format
- `500 Internal Server Error` - Database error

**Sorting**
Results are sorted by `created_at` in **ascending order** (oldest first), making it easier to analyze data chronologically.

**Use Cases**
- **Daily Reports**: Get all speeds for a specific day (00:00:00 to 23:59:59)
  ```
  start_date=2024-01-15&end_date=2024-01-15
  ```
  This will automatically search from 2024-01-15 00:00:00 to 2024-01-15 23:59:59

- **Weekly Analysis**: Get speeds for a week
  ```
  start_date=2024-01-08&end_date=2024-01-14
  ```
  Includes all data from Jan 8th 00:00:00 to Jan 14th 23:59:59

- **Peak Hours**: Get speeds during specific hours (rush hour)
  ```
  start_date=2024-01-15 07:00:00&end_date=2024-01-15 09:00:00
  ```
  Precise time range from 7:00 AM to 9:00 AM

- **Monthly Export**: Export an entire month of data
  ```
  start_date=2024-01-01&end_date=2024-01-31
  ```
  All data from January 1st to January 31st (inclusive)

- **Specific Time Window**: Get data between specific timestamps
  ```
  start_date=2024-01-15 14:30:00&end_date=2024-01-15 18:45:00
  ```
  From 2:30 PM to 6:45 PM on January 15th

**JavaScript/TypeScript Example**

```javascript
// Fetch speeds for a specific date range
async function getSpeedsByDateRange(startDate, endDate) {
  const params = new URLSearchParams({
    start_date: startDate,
    end_date: endDate
  });

  const response = await fetch(`http://localhost:8080/api/speeds/range?${params}`, {
    headers: {
      'Authorization': 'Bearer your_api_token_here'
    }
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  const speeds = await response.json();
  return speeds;
}

// Usage examples
// Get all speeds from January 2024
const januarySpeeds = await getSpeedsByDateRange('2024-01-01', '2024-01-31');

// Get speeds for today
const today = new Date().toISOString().split('T')[0];
const todaySpeeds = await getSpeedsByDateRange(today, today);

// Get speeds for specific time window
const morningSpeeds = await getSpeedsByDateRange(
  '2024-01-15 06:00:00',
  '2024-01-15 12:00:00'
);
```

**Python Example**

```python
import requests
from datetime import datetime, timedelta

def get_speeds_by_date_range(start_date, end_date):
    params = {
        'start_date': start_date,
        'end_date': end_date
    }
    headers = {
        'Authorization': 'Bearer your_api_token_here'
    }
    response = requests.get('http://localhost:8080/api/speeds/range', params=params, headers=headers)
    response.raise_for_status()
    return response.json()

# Get speeds for the last 7 days
end_date = datetime.now()
start_date = end_date - timedelta(days=7)

speeds = get_speeds_by_date_range(
    start_date.strftime('%Y-%m-%d'),
    end_date.strftime('%Y-%m-%d')
)

# Calculate average speed
if speeds:
    avg_speed = sum(s['speed'] for s in speeds) / len(speeds)
    print(f"Average speed over last 7 days: {avg_speed:.2f} km/h")
```

**Notes**
- No limit on the number of results returned (unlike other endpoints)
- For very large date ranges, the query may take longer to execute
- Both `start_date` and `end_date` are **inclusive**
- Times are in UTC timezone
- Invalid date formats will result in a 400 Bad Request error with details
- When providing only a date (without time):
  - `start_date` defaults to **00:00:00** (start of the day)
  - `end_date` defaults to **23:59:59** (end of the day)
  - This ensures the entire day is included in the search
- When providing date with time, the exact timestamp is used
- Empty date range (start > end) will return an empty array

---

### Real-time Speed Stream (SSE)

**`GET /api/speeds/stream`**

Subscribe to real-time speed measurements using Server-Sent Events (SSE). This endpoint establishes a persistent connection and pushes new speed data to clients immediately as sensors submit measurements.

🔒 **Requires Authentication**: This endpoint requires a valid Bearer token in the Authorization header.

**Use Case**
Perfect for real-time dashboards, monitoring applications, and live data visualization without the need for polling.

**Connection**
```bash
# Using curl
curl -N -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds/stream

# Using httpie
http --stream http://localhost:8080/api/speeds/stream \
  Authorization:"Bearer your_api_token_here"
```

**Event Stream Format**
Each new speed measurement is sent as an SSE event:

```
data: {"id":123,"sensor_name":"Highway Sensor 001","speed":75.3,"lane":1,"created_at":"2025-11-25T14:30:00.123456Z"}

data: {"id":124,"sensor_name":"Sensor A","speed":62.1,"lane":0,"created_at":"2025-11-25T14:30:05.789012Z"}
```

**JavaScript/TypeScript Example**

```javascript
// NOTE: EventSource doesn't support custom headers, so we use fetch with ReadableStream
async function connectToSpeedStream(token) {
  const response = await fetch('http://localhost:8080/api/speeds/stream', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  const reader = response.body.getReader();
  const decoder = new TextDecoder();

  while (true) {
    const { done, value } = await reader.read();

    if (done) {
      console.log('Stream ended');
      break;
    }

    // Decode and process SSE data
    const chunk = decoder.decode(value);
    const lines = chunk.split('\n');

    for (const line of lines) {
      if (line.startsWith('data: ')) {
        const data = line.substring(6);
        try {
          const speedData = JSON.parse(data);
          console.log('New speed received:', speedData);
          // Update your UI here
        } catch (e) {
          console.error('Failed to parse speed data:', e);
        }
      }
    }
  }
}

// Usage
connectToSpeedStream('your_api_token_here')
  .catch(error => console.error('Stream error:', error));
```

**React Hook Example**

```typescript
import { useEffect, useState } from 'react';

interface SpeedData {
  id: number;
  sensor_name: string | null;
  speed: number;
  lane: 0 | 1;
  created_at: string;
}

function useSpeedStream(token: string) {
  const [latestSpeed, setLatestSpeed] = useState<SpeedData | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    let reader: ReadableStreamDefaultReader | null = null;
    let aborted = false;

    async function connect() {
      try {
        const response = await fetch('http://localhost:8080/api/speeds/stream', {
          headers: {
            'Authorization': `Bearer ${token}`
          }
        });

        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }

        setIsConnected(true);
        reader = response.body!.getReader();
        const decoder = new TextDecoder();

        while (!aborted) {
          const { done, value } = await reader.read();

          if (done) break;

          const chunk = decoder.decode(value);
          const lines = chunk.split('\n');

          for (const line of lines) {
            if (line.startsWith('data: ')) {
              try {
                const speedData: SpeedData = JSON.parse(line.substring(6));
                setLatestSpeed(speedData);
              } catch (e) {
                console.error('Failed to parse speed data:', e);
              }
            }
          }
        }
      } catch (error) {
        console.error('Stream error:', error);
        setIsConnected(false);
      }
    }

    connect();

    return () => {
      aborted = true;
      reader?.cancel();
      setIsConnected(false);
    };
  }, [token]);

  return { latestSpeed, isConnected };
}

// Usage in component
function Dashboard() {
  const { latestSpeed, isConnected } = useSpeedStream('your_api_token_here');

  return (
    <div>
      <p>Status: {isConnected ? 'Connected' : 'Disconnected'}</p>
      {latestSpeed && (
        <div>
          <p>Speed: {latestSpeed.speed} km/h</p>
          <p>Lane: {latestSpeed.lane === 0 ? 'Left' : 'Right'}</p>
        </div>
      )}
    </div>
  );
}
```

**Status Codes**
- `200 OK` - SSE connection established successfully
- Connection remains open indefinitely until client disconnects

**Features**
- **Zero Latency**: Measurements are pushed immediately when received
- **Auto-reconnect**: Browser automatically reconnects if connection drops
- **No Polling**: Eliminates the need for repeated API calls
- **Multiple Clients**: Supports unlimited concurrent connections
- **Efficient**: Uses HTTP/1.1 chunked transfer encoding

**Performance Notes**
- The broadcast channel has a capacity of 100 messages
- If a slow client can't keep up, older messages are dropped to prevent memory issues
- Connection stays open indefinitely (no timeout)
- CORS is enabled for cross-origin connections

**Browser Compatibility**
Server-Sent Events are supported in all modern browsers:
- ✅ Chrome/Edge 6+
- ✅ Firefox 6+
- ✅ Safari 5+
- ✅ Opera 11+
- ❌ Internet Explorer (use polyfill)

---

## Error Responses

All endpoints may return error responses in the following format:

**500 Internal Server Error**
```
Status: 500 Internal Server Error
```

**400 Bad Request** (for POST requests with invalid data)
```
Status: 400 Bad Request
```

---

## Data Models

### SpeedData
```typescript
interface SpeedData {
  id: number;                    // Unique identifier
  sensor_name: string | null;    // Optional sensor name
  speed: number;                 // Speed in km/h (float)
  lane: 0 | 1;                   // 0 = Left lane, 1 = Right lane
  created_at: string;            // ISO 8601 datetime in UTC
}
```

### Lane Values
| Value | Description |
|-------|-------------|
| `0` | Left lane |
| `1` | Right lane |

---

## Performance & Caching

The API implements multiple performance optimization strategies:

### Redis Caching (Cache-Aside Pattern)

1. **Read Operations** (`GET /api/speeds/latest`):
   - First checks Redis cache
   - If cache miss, queries PostgreSQL
   - Stores result in Redis with 1-hour TTL

2. **Write Operations** (`POST /api/speeds`):
   - Writes to PostgreSQL database
   - Immediately updates Redis cache
   - Ensures cache consistency

This provides significant performance improvements for frequently accessed data, especially for the latest speed measurement endpoint.

### Real-time Broadcasting

The API uses an in-memory broadcast channel for real-time notifications:

1. **Write Operations** (`POST /api/speeds`):
   - After database write and cache update
   - Broadcasts speed data to all connected SSE clients
   - Zero latency notification delivery

2. **SSE Connections** (`GET /api/speeds/stream`):
   - Each client subscribes to the broadcast channel
   - No polling overhead
   - Efficient memory usage with 100-message capacity
   - Supports unlimited concurrent connections

---

## Rate Limiting

Currently, no rate limiting is implemented. This may be added in future versions.

---

## CORS

CORS is enabled for all origins (`permissive` mode). In production, you should restrict this to specific allowed origins.

---

## Examples

### Complete Workflow Example

```bash
# 1. Check API health (no authentication required)
curl http://localhost:8080/health

# 2. Create a new speed measurement
curl -X POST http://localhost:8080/api/speeds \
  -H "Authorization: Bearer your_api_token_here" \
  -H "Content-Type: application/json" \
  -d '{"sensor_name": "Highway 101 North", "speed": 72.5, "lane": 1}'

# 3. Get the latest measurement (cached in Redis)
curl -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds/latest

# 4. Get last 10 measurements
curl -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds?limit=10

# 5. Get today's measurements
curl -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds/today

# 6. Get measurements with pagination
curl -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds/paginated?offset=0&limit=25

# 7. Get measurements by date range
curl -H "Authorization: Bearer your_api_token_here" \
  "http://localhost:8080/api/speeds/range?start_date=2024-01-01&end_date=2024-01-31"

# 8. Subscribe to real-time updates (SSE)
curl -N -H "Authorization: Bearer your_api_token_here" \
  http://localhost:8080/api/speeds/stream
# This will keep the connection open and display new measurements as they arrive
```

### Arduino/IoT Device Example

```cpp
// Arduino example for posting speed data
#include <WiFi.h>
#include <HTTPClient.h>

void sendSpeedData(float speed, int lane) {
  HTTPClient http;
  http.begin("http://your-server:8080/api/speeds");
  http.addHeader("Content-Type", "application/json");
  http.addHeader("Authorization", "Bearer your_api_token_here");

  String payload = "{\"sensor_name\":\"Arduino-001\",\"speed\":" +
                   String(speed) + ",\"lane\":" + String(lane) + "}";

  int httpResponseCode = http.POST(payload);

  if (httpResponseCode == 201) {
    Serial.println("Speed data sent successfully");
  } else {
    Serial.print("Error sending data. HTTP code: ");
    Serial.println(httpResponseCode);
  }

  http.end();
}
```

---

## Migration from Old Endpoints

If you're migrating from the previous version, here's the mapping:

| Old Endpoint | New Endpoint | Method |
|-------------|--------------|--------|
| `/api/create-speed` | `/api/speeds` | POST |
| `/api/get-speed` | `/api/speeds` | GET |
| `/api/get-speed/last` | `/api/speeds/latest` | GET |
| `/api/get-speed/today` | `/api/speeds/today` | GET |
| `/api/get-speed/pagination` | `/api/speeds/paginated` | GET |

---

## Support

For issues, questions, or contributions, please visit:
- GitHub: https://github.com/Maxime-Cllt/SpeedStream
- License: GPL-3.0
