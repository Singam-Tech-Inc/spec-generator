# Task Examples

This document shows example taskfiles from various API specifications to help understand task structure and story point estimation.

## REST API Example: User Service

### SPEC-00001-user-service

Specification for a user management REST API.

---

### 01-feature-user-endpoints.md

```markdown
---
title: "User Management Endpoints"
story_points: 5
status: "pending"
---

# User Management Endpoints

Implement core REST endpoints for user CRUD operations.

## Description

Create endpoints for:
- GET /users - List all users
- GET /users/{id} - Get single user
- POST /users - Create new user
- PUT /users/{id} - Update user
- DELETE /users/{id} - Delete user

## Acceptance Criteria

- [ ] All endpoints return proper JSON responses
- [ ] Correct HTTP status codes (200, 201, 204, 400, 404)
- [ ] Request validation works
- [ ] Pagination implemented for list endpoint
- [ ] Proper error messages returned

## Implementation Notes

- Use standard REST conventions
- Implement request validation middleware
- Add request ID tracking
- Handle concurrency properly
```

### 02-feature-authentication.md

```markdown
---
title: "OAuth2 Authentication"
story_points: 8
status: "pending"
---

# OAuth2 Authentication

Implement OAuth2 authentication with JWT tokens.

## Description

- Authorization code flow with PKCE
- JWT token generation and validation
- Token refresh mechanism
- Scope-based access control

## Acceptance Criteria

- [ ] Authorization endpoint implemented
- [ ] Token endpoint returns JWT
- [ ] Token refresh works
- [ ] Access tokens expire correctly
- [ ] Invalid tokens rejected with 401
- [ ] Scopes enforced on endpoints

## Implementation Notes

- Use industry-standard JWT library
- Implement token rotation
- Add CORS handling for OAuth flow
- Secure token storage in headers
```

### 03-test-plan.md

```markdown
---
title: "Test Plan: User API"
story_points: 8
status: "pending"
---

# Test Plan: User API

Comprehensive testing strategy for user management API.

## Unit Tests

### User Model Tests
- [ ] Validate email format
- [ ] Hash password correctly
- [ ] Set creation/update timestamps
- [ ] Validate required fields
- [ ] Handle edge cases (empty strings, special chars)

### Endpoint Tests
- [ ] GET /users returns 200 with list
- [ ] GET /users/{id} returns 200 when exists
- [ ] GET /users/{id} returns 404 when not found
- [ ] POST /users creates and returns 201
- [ ] POST /users rejects invalid data with 400
- [ ] PUT /users/{id} updates and returns 200
- [ ] DELETE /users/{id} removes user

## Integration Tests

### Database Integration
- [ ] Users persisted to database
- [ ] Queries return correct data
- [ ] Updates reflected immediately
- [ ] Deletions remove data
- [ ] Concurrent writes handled correctly

### Authentication Integration
- [ ] Protected endpoints require auth
- [ ] Invalid tokens rejected
- [ ] Scope checking works

## E2E Tests (via API client)

### User Flow
- [ ] User signs up (OAuth)
- [ ] Token received and stored
- [ ] Create user resource via API
- [ ] Retrieve created user
- [ ] Update user
- [ ] Delete user
- [ ] Verify deletion

## Load/Performance Tests

- [ ] 1000 requests/sec sustained
- [ ] P99 latency < 100ms
- [ ] Error rate < 0.1%
- [ ] Database connection pooling works

## Security Tests

- [ ] No SQL injection via input
- [ ] CORS properly configured
- [ ] Rate limiting enforced
- [ ] Sensitive data not logged
```

### 04-implementation.md

```markdown
---
title: "Core Implementation"
story_points: 13
status: "pending"
---

# Core Implementation

Implement the user service core functionality.

## Setup

- [ ] Initialize Express/FastAPI/Spring Boot project
- [ ] Configure database connection
- [ ] Set up environment variables
- [ ] Create logging infrastructure
- [ ] Set up middleware pipeline

## User Model

- [ ] Define user schema
- [ ] Add field validations
- [ ] Implement password hashing
- [ ] Create model tests

## Data Access

- [ ] Implement database queries
- [ ] Create connection pooling
- [ ] Add transaction handling
- [ ] Implement data migration script
- [ ] Create database backup strategy

## API Implementation

- [ ] Implement request/response serialization
- [ ] Add request validation middleware
- [ ] Implement error handling middleware
- [ ] Add logging for all requests
- [ ] Implement rate limiting

## Error Handling

- [ ] Database error handling
- [ ] Validation error handling
- [ ] Authentication error handling
- [ ] Generic error responses
- [ ] Error logging and monitoring

## Code Quality

- [ ] Code review completed
- [ ] All tests passing (>80% coverage)
- [ ] Linting passed
- [ ] Documentation complete
```

### 05-documentation.md

```markdown
---
title: "API Documentation"
story_points: 5
status: "pending"
---

# API Documentation

Create comprehensive API documentation.

## Endpoint Documentation

- [ ] Document each endpoint
- [ ] Show request examples
- [ ] Show response examples
- [ ] List error codes
- [ ] Document parameters

## OpenAPI Specification

- [ ] Generate OpenAPI 3.0 spec
- [ ] Validate spec
- [ ] Host interactive documentation (Swagger UI)
- [ ] Create Postman collection

## Developer Guide

- [ ] Setup instructions
- [ ] Authentication guide
- [ ] Rate limiting guide
- [ ] Error handling guide
- [ ] Best practices

## Deployment Guide

- [ ] Environment setup
- [ ] Configuration options
- [ ] Database setup
- [ ] Monitoring setup
- [ ] Scaling recommendations

## Changelog

- [ ] Document version 1.0.0 release
- [ ] List all features
- [ ] Note breaking changes
```

---

## GraphQL API Example: Product Catalog

### SPEC-00002-product-catalog

Specification for a GraphQL product catalog API.

---

### 01-schema-definition.md

```markdown
---
title: "GraphQL Schema Definition"
story_points: 8
status: "pending"
---

# GraphQL Schema Definition

Define the GraphQL schema for product catalog.

## Types to Implement

- Product (id, name, description, price, inventory)
- Category (id, name, products)
- Query root (products, product, categories, search)
- Mutation root (createProduct, updateProduct, deleteProduct)

## Acceptance Criteria

- [ ] Schema is valid GraphQL
- [ ] All types have proper fields
- [ ] Relationships properly connected
- [ ] Input types defined for mutations
- [ ] Resolvers return correct data
- [ ] Introspection query works

## Implementation Notes

- Use GraphQL best practices
- Implement proper error types
- Add pagination for list queries
- Support filtering and sorting
```

### 02-implementation.md

```markdown
---
title: "GraphQL Resolvers"
story_points: 13
status: "pending"
---

# GraphQL Resolvers

Implement resolver functions for GraphQL schema.

## Query Resolvers

- [ ] products(filter, limit, offset) - list products
- [ ] product(id) - get single product
- [ ] categories() - list categories
- [ ] search(query) - search products

## Mutation Resolvers

- [ ] createProduct(input) - create product
- [ ] updateProduct(id, input) - update product
- [ ] deleteProduct(id) - delete product

## Subscription Resolvers (if needed)

- [ ] productCreated - notify on new product
- [ ] productUpdated - notify on product change

## Performance

- [ ] Implement query complexity analysis
- [ ] Add caching strategy
- [ ] Batch data loading (DataLoader)
- [ ] N+1 query prevention
```

---

## Microservice Example: Order Service

### SPEC-00005-order-service

Specification for an order processing microservice.

---

### 01-service-contract.md

```markdown
---
title: "Order Service Contract"
story_points: 5
status: "pending"
---

# Order Service Contract

Define the service contract and dependencies.

## Service Dependencies

- [ ] User Service (verify user exists)
- [ ] Product Service (check inventory)
- [ ] Payment Service (process payment)
- [ ] Notification Service (send confirmations)

## API Contract

- [ ] Document service endpoints
- [ ] Define request/response formats
- [ ] Specify error codes
- [ ] Document retry policy
- [ ] Define timeout values

## Events Produced

- [ ] order.created
- [ ] order.payment_processed
- [ ] order.shipped
- [ ] order.cancelled

## Configuration

- [ ] Service discovery config
- [ ] Environment variables
- [ ] Feature flags
- [ ] Circuit breaker settings
```

### 02-event-integration.md

```markdown
---
title: "Event Integration"
story_points: 8
status: "pending"
---

# Event Integration

Implement event-driven communication with other services.

## Event Publishing

- [ ] Publish order.created event
- [ ] Publish order.shipped event
- [ ] Implement event serialization
- [ ] Add event tracking
- [ ] Implement dead-letter queue

## Event Consuming

- [ ] Subscribe to payment.processed
- [ ] Subscribe to inventory.reserved
- [ ] Handle event failures
- [ ] Implement idempotency
- [ ] Add event logging

## Monitoring

- [ ] Event publishing metrics
- [ ] Event processing latency
- [ ] Error rate monitoring
- [ ] Dead-letter queue monitoring
```

---

## Story Points Reference

### Fibonacci Scale Usage

- **1 point** - Trivial task (documentation update, config change)
- **2 points** - Small task (single function, simple test)
- **3 points** - Small-medium task (multiple related functions)
- **5 points** - Medium task (feature with basic testing)
- **8 points** - Medium-large task (feature with comprehensive testing)
- **13 points** - Large task (complex feature or major component)
- **21 points** - Very large task (multiple features or service)

### Estimation Tips

- **Under 5 points**: Can complete in 1 day or less
- **5-8 points**: Typically 2-3 days of work
- **13+ points**: Likely needs to be split further

---

## Task Breakdown Strategy

### By Feature

Break down by API features:
- Each endpoint/query → one task
- Related operations → one task

### By Layer

Break down by technical layers:
- API/Schema definition
- Business logic
- Data access
- Tests
- Documentation

### By Concern

Break down by cross-cutting concerns:
- Core functionality
- Error handling
- Performance optimization
- Security
- Monitoring

---

For more details on creating taskfiles, see the template-guide.md and default-template.md in this skill's references.
