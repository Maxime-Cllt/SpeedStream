# SpeedStream API Documentation

## Base URL
```
http://localhost:3000
```

## Table of Contents
- [Health Check](#health-check)
- [Speed Measurements](#speed-measurements)
  - [Create Speed Measurement](#create-speed-measurement)
  - [Get Speed Measurements](#get-speed-measurements)
  - [Get Latest Speed](#get-latest-speed)
  - [Get Today's Speeds](#get-todays-speeds)
  - [Get Paginated Speeds](#get-paginated-speeds)

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
curl -X POST http://localhost:3000/api/speeds \
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

**Query Parameters**
| Parameter | Type | Default | Max | Description |
|-----------|------|---------|-----|-------------|
| `limit` | integer | 100 | 1000 | Number of records to retrieve |

**Example Request**
```bash
curl http://localhost:3000/api/speeds?limit=50
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

**Example Request**
```bash
curl http://localhost:3000/api/speeds/latest
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

**Query Parameters**
| Parameter | Type | Default | Max | Description |
|-----------|------|---------|-----|-------------|
| `limit` | integer | 100 | 1000 | Maximum number of records to retrieve |

**Example Request**
```bash
curl http://localhost:3000/api/speeds/today?limit=200
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

**Query Parameters**
| Parameter | Type | Default | Max | Description |
|-----------|------|---------|-----|-------------|
| `offset` | integer | 0 | - | Number of records to skip |
| `limit` | integer | 100 | 1000 | Number of records to retrieve |

**Example Request**
```bash
# Get records 100-149 (page 2 with 50 items per page)
curl http://localhost:3000/api/speeds/paginated?offset=100&limit=50
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

fetch(`http://localhost:3000/api/speeds/paginated?offset=${offset}&limit=${itemsPerPage}`)
  .then(response => response.json())
  .then(data => console.log(data));
```

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

The API implements a **cache-aside** strategy using Redis:

1. **Read Operations** (`GET /api/speeds/latest`):
   - First checks Redis cache
   - If cache miss, queries PostgreSQL
   - Stores result in Redis with 1-hour TTL

2. **Write Operations** (`POST /api/speeds`):
   - Writes to PostgreSQL database
   - Immediately updates Redis cache
   - Ensures cache consistency

This provides significant performance improvements for frequently accessed data, especially for the latest speed measurement endpoint.

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
# 1. Check API health
curl http://localhost:3000/health

# 2. Create a new speed measurement
curl -X POST http://localhost:3000/api/speeds \
  -H "Content-Type: application/json" \
  -d '{"sensor_name": "Highway 101 North", "speed": 72.5, "lane": 1}'

# 3. Get the latest measurement (cached in Redis)
curl http://localhost:3000/api/speeds/latest

# 4. Get last 10 measurements
curl http://localhost:3000/api/speeds?limit=10

# 5. Get today's measurements
curl http://localhost:3000/api/speeds/today

# 6. Get measurements with pagination
curl http://localhost:3000/api/speeds/paginated?offset=0&limit=25
```

### Arduino/IoT Device Example

```cpp
// Arduino example for posting speed data
#include <WiFi.h>
#include <HTTPClient.h>

void sendSpeedData(float speed, int lane) {
  HTTPClient http;
  http.begin("http://your-server:3000/api/speeds");
  http.addHeader("Content-Type", "application/json");

  String payload = "{\"sensor_name\":\"Arduino-001\",\"speed\":" +
                   String(speed) + ",\"lane\":" + String(lane) + "}";

  int httpResponseCode = http.POST(payload);

  if (httpResponseCode == 201) {
    Serial.println("Speed data sent successfully");
  } else {
    Serial.println("Error sending data");
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
