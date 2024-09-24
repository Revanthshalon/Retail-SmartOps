### User Access Management System Design

#### Overview

This document outlines the design of the user access management system, detailing the relationships between users,
roles, permissions, stores, and other entities.

#### Entities and Relationships

1. **User**
    - Represents a user in the system.
    - Fields: `id`, `username`, `password`, `email`, `created_at`, `updated_at`, `is_active`.

2. **Role**
    - Represents a role in the system.
    - Fields: `id`, `name`.

3. **Permission**
    - Represents a permission in the system.
    - Fields: `id`, `name`, `can_read`, `can_write`, `can_delete`, `can_update`.

4. **UserRole**
    - Represents the relationship between users and their roles.
    - Fields: `user_id`, `role_id`, `assigned_at`.

5. **RolePermission**
    - Represents the relationship between roles and their associated permissions.
    - Fields: `role_id`, `permission_id`.

6. **Store**
    - Represents a store in the system.
    - Fields: `id`, `owner_id`, `name`, `address`, `created_at`, `updated_at`.

7. **Address**
    - Represents an address in the system.
    - Fields: `country`, `state`, `city`, `street`, `zip`.

8. **StoreUsers**
    - Represents the relationship between stores and their associated users.
    - Fields: `store_id`, `user_id`.

9. **UserHierarchy**
    - Represents the reporting structure of users.
    - Fields: `user_id`, `reports_to`.

#### Entity Relationships

- **User and Role**
    - A user can have multiple roles.
    - A role can be assigned to multiple users.
    - Relationship: Many-to-Many (via `UserRole`).

- **Role and Permission**
    - A role can have multiple permissions.
    - A permission can be assigned to multiple roles.
    - Relationship: Many-to-Many (via `RolePermission`).

- **Store and User**
    - A store can have multiple associated users.
    - A user can be associated with multiple stores.
    - Relationship: Many-to-Many (via `StoreUsers`).

- **User and UserHierarchy**
    - A user can report to another user.
    - Relationship: One-to-Many (self-referencing via `UserHierarchy`).

#### Example Data Flow

1. **User Creation**
    - A new user is created with specific details (username, password, email, etc.).
    - The user is assigned one or more roles via the `UserRole` entity.

2. **Role Assignment**
    - Roles are defined with specific permissions.
    - Permissions are assigned to roles via the `RolePermission` entity.

3. **Store Management**
    - Stores are created with specific details (name, address, etc.).
    - Users are associated with stores via the `StoreUsers` entity.

4. **User Reporting Structure**
    - Users are assigned to report to other users via the `UserHierarchy` entity.

This design ensures a flexible and scalable user access management system, allowing for detailed control over user
permissions and roles within the system.