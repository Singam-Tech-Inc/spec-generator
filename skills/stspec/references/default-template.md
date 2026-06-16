# {{ SPEC_TITLE }}

**Specification ID**: {{ SPEC_ID }}  
**Version**: 1.0.0  
**Created**: {{ CREATED_DATE }}  
**Status**: Draft  

## Overview

{{ SPEC_DESCRIPTION }}

### Quick Facts

- **API Type**: REST / GraphQL / WebSocket (specify)
- **Base URL**: `https://api.example.com/v1`
- **Authentication**: OAuth2 / JWT / API Key (specify)
- **Response Format**: JSON

## Endpoints

### Base Structure

All endpoints follow this pattern:
```
[METHOD] /v1/[resource]/[action]
```

### Endpoint Template

#### [HTTP Method] /resource/endpoint

**Description**: What this endpoint does

**Path Parameters**:
- `id` (string, required): Resource identifier
- `type` (string, optional): Filter type

**Query Parameters**:
- `limit` (integer, optional, default: 20): Number of results
- `offset` (integer, optional, default: 0): Pagination offset
- `sort` (string, optional): Sort order (asc/desc)

**Request Headers**:
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Request Body**:
```json
{
  "field_name": "value",
  "nested_object": {
    "key": "value"
  }
}
```

**Response** (200 OK):
```json
{
  "id": "resource-id",
  "status": "success",
  "data": {
    "field_name": "value",
    "created_at": "2024-01-15T10:30:00Z"
  },
  "meta": {
    "request_id": "req-12345"
  }
}
```

**Response** (400 Bad Request):
```json
{
  "status": "error",
  "code": "INVALID_REQUEST",
  "message": "Description of what went wrong",
  "details": {
    "field": ["Error details"]
  }
}
```

**Curl Example**:
```bash
curl -X [METHOD] https://api.example.com/v1/resource/endpoint \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "field_name": "value"
  }'
```

---

## Request/Response Format

### Request Structure

All requests must include:
- **Authentication**: Bearer token in Authorization header
- **Content-Type**: application/json (for POST/PUT/PATCH)
- **Custom Headers**: (if applicable)
  - `X-Request-ID`: Optional unique request identifier
  - `X-API-Version`: Optional API version override

### Response Structure

All responses follow this envelope:

```json
{
  "status": "success|error|pending",
  "code": "SUCCESS|ERROR_CODE|WARNING_CODE",
  "message": "Human-readable message",
  "data": { /* Response payload */ },
  "meta": {
    "request_id": "unique-request-id",
    "timestamp": "2024-01-15T10:30:00Z",
    "api_version": "1.0.0"
  },
  "errors": [
    {
      "field": "field_name",
      "code": "VALIDATION_ERROR",
      "message": "Specific error details"
    }
  ]
}
```

### Pagination

For endpoints returning lists:

```json
{
  "status": "success",
  "data": [
    { /* items */ }
  ],
  "pagination": {
    "total": 500,
    "limit": 20,
    "offset": 0,
    "has_more": true,
    "pages": 25
  }
}
```

---

## Error Codes and Status Codes

### HTTP Status Codes

| Code | Name | Description |
|------|------|-------------|
| 200 | OK | Successful request |
| 201 | Created | Resource created successfully |
| 204 | No Content | Successful but no response body |
| 400 | Bad Request | Invalid request parameters |
| 401 | Unauthorized | Authentication required or failed |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Resource conflict (e.g., duplicate) |
| 422 | Unprocessable Entity | Validation failed |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |
| 503 | Service Unavailable | Service temporarily unavailable |

### API Error Codes

| Code | Status | Description | Retry? |
|------|--------|-------------|--------|
| INVALID_REQUEST | 400 | Request validation failed | No |
| AUTHENTICATION_FAILED | 401 | Authentication token invalid/expired | Yes |
| INSUFFICIENT_PERMISSIONS | 403 | User lacks required permissions | No |
| RESOURCE_NOT_FOUND | 404 | Requested resource doesn't exist | No |
| RESOURCE_CONFLICT | 409 | Resource already exists | No |
| VALIDATION_ERROR | 422 | One or more fields failed validation | No |
| RATE_LIMIT_EXCEEDED | 429 | Rate limit reached | Yes (after delay) |
| INTERNAL_ERROR | 500 | Unexpected server error | Yes |
| SERVICE_UNAVAILABLE | 503 | Service temporarily down | Yes |

**Error Response Example**:
```json
{
  "status": "error",
  "code": "VALIDATION_ERROR",
  "message": "Request validation failed",
  "errors": [
    {
      "field": "email",
      "code": "INVALID_FORMAT",
      "message": "Invalid email format"
    },
    {
      "field": "password",
      "code": "TOO_SHORT",
      "message": "Password must be at least 8 characters"
    }
  ]
}
```

---

## Authentication & Authorization

### OAuth2

**Flow**: Authorization Code with PKCE

**Authorization Endpoint**:
```
GET /oauth/authorize?
  client_id=YOUR_CLIENT_ID&
  redirect_uri=YOUR_REDIRECT_URI&
  scope=read write&
  state=RANDOM_STATE&
  code_challenge=CHALLENGE&
  code_challenge_method=S256
```

**Token Endpoint**:
```
POST /oauth/token
Content-Type: application/x-www-form-urlencoded

grant_type=authorization_code&
code=AUTHORIZATION_CODE&
client_id=YOUR_CLIENT_ID&
code_verifier=VERIFIER
```

**Token Response**:
```json
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "refresh_token": "refresh_token_value",
  "scope": "read write"
}
```

**Scopes**:
- `read` - Read-only access
- `write` - Write/modify access
- `admin` - Administrative access

### API Key Authentication

Include in request headers:
```
Authorization: Bearer YOUR_API_KEY
```

Or as query parameter:
```
GET /endpoint?api_key=YOUR_API_KEY
```

### JWT Token

**Header**:
```
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Payload**:
```json
{
  "sub": "user-id",
  "iat": 1642254600,
  "exp": 1642258200,
  "scope": "read write"
}
```

---

## Rate Limiting

### Rate Limit Headers

All responses include rate limit information:

```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1642258200
```

### Limits

- **Default**: 1,000 requests per hour per user
- **Premium**: 10,000 requests per hour
- **Enterprise**: Custom limits

### Rate Limit Exceeded

When limit is exceeded:
```
HTTP/1.1 429 Too Many Requests
X-RateLimit-Reset: 1642258200

{
  "status": "error",
  "code": "RATE_LIMIT_EXCEEDED",
  "message": "Rate limit exceeded",
  "retry_after": 3600
}
```

---

## API Versioning

### Current Version: 1.0.0

**Version Header** (optional):
```
X-API-Version: 1.0.0
```

### Deprecation

Deprecated endpoints will include:
```
Deprecation: true
Sunset: 2025-01-01T00:00:00Z
Link: <https://docs.example.com/migration>; rel="deprecation"
```

---

## Webhooks (if applicable)

### Webhook Events

Supported event types:
- `resource.created`
- `resource.updated`
- `resource.deleted`
- `action.completed`
- `action.failed`

### Webhook Payload

```json
{
  "event": "resource.created",
  "timestamp": "2024-01-15T10:30:00Z",
  "data": {
    "id": "resource-id",
    "type": "resource_type",
    "attributes": { /* Resource data */ }
  },
  "retry_count": 0
}
```

### Delivery

- **Retries**: Up to 5 attempts over 24 hours
- **Timeout**: 30 seconds per attempt
- **Verification**: Include `X-Signature` header with HMAC-SHA256

---

## Additional Resources

### Documentation Links

- [API Reference Docs](https://docs.example.com)
- [Authentication Guide](https://docs.example.com/auth)
- [Rate Limiting Guide](https://docs.example.com/rate-limiting)
- [Migration Guides](https://docs.example.com/migration)

### Support

- **Email**: api-support@example.com
- **Slack**: #api-support
- **Status Page**: https://status.example.com
- **Issue Tracker**: https://github.com/example/api-issues

---

## Changelog

### Version 1.0.0 ({{ CREATED_DATE }})
- Initial API specification
- [List major features]

---

**Document Status**: Draft  
**Last Updated**: {{ CREATED_DATE }}  
**Maintained By**: [Team/Owner]
