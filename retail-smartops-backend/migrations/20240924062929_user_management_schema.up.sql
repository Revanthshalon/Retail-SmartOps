/*
====================================================================================================================
=========================== Migration script for creating user_management schema ===================================
====================================================================================================================
 */

/* Enabling Uuid */
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

/* Create Users Table */
CREATE TABLE users
(
    id         UUID      DEFAULT uuid_generate_v4(),
    username   VARCHAR(50) UNIQUE NOT NULL,
    password   VARCHAR(50)        NOT NULL,
    email      VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active  BOOLEAN   DEFAULT TRUE,
    PRIMARY KEY (id)
);

/* Create Roles Table */
CREATE TABLE roles
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(30) UNIQUE NOT NULL
);

/* Create User_Role Table */
CREATE TABLE user_roles
(
    user_id     UUID REFERENCES users (id) ON DELETE CASCADE,
    role_id     SERIAL REFERENCES roles (id) ON DELETE CASCADE,
    assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, role_id)
);

/* Create Permissions Table */
CREATE TABLE permissions
(
    id          SERIAL PRIMARY KEY,
    entity_name VARCHAR(100) NOT NULL,
    can_read    BOOLEAN DEFAULT TRUE,
    can_write   BOOLEAN DEFAULT FALSE,
    can_delete  BOOLEAN DEFAULT FALSE,
    can_update  BOOLEAN DEFAULT FALSE
);

/* Create Role_Permission Table */
CREATE TABLE role_permissions
(
    role_id       INT REFERENCES roles (id) ON DELETE CASCADE,
    permission_id INT REFERENCES permissions (id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

/* Create Stores Table */
CREATE TABLE stores
(
    store_id   SERIAL PRIMARY KEY,
    store_name VARCHAR(255) NOT NULL,
    country    VARCHAR(50)  NOT NULL,
    state      VARCHAR(50)  NOT NULL,
    city       VARCHAR(50)  NOT NULL,
    street     VARCHAR(50)  NOT NULL,
    zip        VARCHAR(50)  NOT NULL,
    owner_id   UUID REFERENCES users (id), -- Each store has an owner who is linked from the users table
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

/* Create Store_Users Table */
CREATE TABLE store_users
(
    store_id SERIAL REFERENCES stores (store_id),
    user_id  UUID REFERENCES users (id),
    PRIMARY KEY (store_id, user_id)
);

/* Create User_Hierarchy Table */
CREATE TABLE user_hierarchy
(
    user_id    UUID REFERENCES users (id),
    reports_to UUID REFERENCES users (id),
    PRIMARY KEY (user_id, reports_to)
);