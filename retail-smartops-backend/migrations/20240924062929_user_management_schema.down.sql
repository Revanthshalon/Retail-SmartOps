/*
====================================================================================================================
=========================== Migration script for dropping user_management schema ===================================
====================================================================================================================
*/

/* Drop User_Hierarchy Table */
DROP TABLE IF EXISTS user_hierarchy;

/* Drop Store_Users Table */
DROP TABLE IF EXISTS store_users;

/* Drop Stores Table */
DROP TABLE IF EXISTS stores;

/* Drop Role_Permissions Table */
DROP TABLE IF EXISTS role_permissions;

/* Drop Permissions Table */
DROP TABLE IF EXISTS permissions;

/* Drop User_Role Table */
DROP TABLE IF EXISTS user_role;

/* Drop Roles Table */
DROP TABLE IF EXISTS roles;

/* Drop Users Table */
DROP TABLE IF EXISTS users;