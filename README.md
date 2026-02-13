# pragmatic_hexagonal
A pragmatic, beginner-friendly guide to Hexagonal Architecture (Ports & Adapters). Decouple your business logic from infrastructure without over-engineering.

## What this repository is NOT about
- It is not meant to be a proper boilerplate or usable template for production. It is meant to be a learning tool, and as such, it is intentionally simplified and focused on the core concepts of Hexagonal Architecture.
- It is not meant to be a comprehensive guide to microservices architecture, the subject is focused on Hexagonal Architure.

### Hexagonal Architecture Structure
Adapters:
- Repositories > Everything related to technical logic implementation

Domains (technical and business application layer):
- Entity > Business entities
- Ports > Methods that adapters must implement (technical logic = create / find / delete / etc...)
- Services > Business logic (plant / collect dead flowers / etc...)

Infrastructure:
- Shared tools (technical, not business-related), e.g.: logger, security, etc...
    They can be used by main, adapters, or domains.

Application:
- Does the same thing as business services (except it's not limited to business logic), but they are called UseCases.
    They can orchestrate one or more business services, as well as calls to non-business adapters.

### Summary
The end goal is that at the top of the codebase, there should only be three types of calls:
- Services (business logic only)
- Adapters (technical logic only)
- Use cases (which orchestrate both)

The best micro-service in this repository to understand hexagonal architecture is inventory_management

In practice, we will almost always have use cases, because we often need to orchestrate both technical and business calls.

### Proposed Team Conventions
- All uses of internal libraries within micro-services must be initialized ONLY in the infrastructure folder, intended for shared tools.
- Consequently, EVERYTHING in the domains, application, and adapters folders is internal and will never be visible from outside the service.

### Separation of Concerns & Coupling
- Each service has write access only to its own database.
- From outside a service, the intended public API is the application/use-cases entry point.
    Ports are internal contracts used inside the service to decouple application/domain from adapters.
- Each database can use its own database technology; if one service uses MongoDB, the other services don't need to know.
- It is impossible for a service to connect to a database that isn't its own with more than read-only access.

---

### Schema explanation (Text version)

```text
👤 CLIENT                           🚚 FOURNISSEUR
          │                                      ▲
          │ (REST / GraphQL)                     │ (Tel / Mail)
          ▼                                      │
+---------------------+           +--------------------------+
|  💻  API GATEWAY    |<--------->|  📦  GESTION STOCKAGE    |
|      (Front)        |   gRPC    |      (Inventory)         |
+---------------------+           +--------------------------+
    │            │                        │        ▲
    │            │ (gRPC)                 └──┐  DB │
    │            ▼                           ▼ ────┘
    │    +---------------------+
    │    |  🚚 LIVRAISON       |─── DB ──┐
    │    |     (Delivery)      |<──┐     │
    │    +---------------------+   └─────┘
    │            ▲
    │            │ (RabbitMQ / Event)
    │            ▼
    │    +---------------------+
    └--->|  💳 FACTURATION     |─── DB ──┐
         |     (Billing)       |<──┐     │
         +---------------------+   └─────┘
```

### Schema explanation (tldraw version)
[tldraw_schema](./logic_schema.tldr)
