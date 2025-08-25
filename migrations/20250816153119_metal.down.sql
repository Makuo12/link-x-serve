-- Add down migration script here
-- === Drop foreign keys first ===
ALTER TABLE "payments" DROP CONSTRAINT IF EXISTS payments_user_id_fkey;
ALTER TABLE "payments" DROP CONSTRAINT IF EXISTS payments_customer_id_fkey;
ALTER TABLE "payments" DROP CONSTRAINT IF EXISTS payments_device_id_fkey;
ALTER TABLE "customers" DROP CONSTRAINT IF EXISTS customers_bank_id_fkey;
ALTER TABLE "devices" DROP CONSTRAINT IF EXISTS devices_business_id_fkey;
ALTER TABLE "devices" DROP CONSTRAINT IF EXISTS devices_account_id_fkey;
ALTER TABLE "devices" DROP CONSTRAINT IF EXISTS devices_id_fkey;
ALTER TABLE "devices_accessible" DROP CONSTRAINT IF EXISTS devices_accessible_user_id_fkey;
ALTER TABLE "businesses" DROP CONSTRAINT IF EXISTS businesses_user_id_fkey;
ALTER TABLE "banks" DROP CONSTRAINT IF EXISTS banks_user_id_fkey;
ALTER TABLE "sessions" DROP CONSTRAINT IF EXISTS sessions_user_id_fkey;
-- === Drop tables (reverse creation order) ===
DROP TABLE IF EXISTS "payments";
DROP TABLE IF EXISTS "customers";
DROP TABLE IF EXISTS "devices";
DROP TABLE IF EXISTS "devices_accessible";
DROP TABLE IF EXISTS "accounts";
DROP TABLE IF EXISTS "businesses";
DROP TABLE IF EXISTS "banks";
DROP TABLE IF EXISTS "sessions";
DROP TABLE IF EXISTS "users";
-- === Drop extension (optional) ===
DROP EXTENSION IF EXISTS "uuid-ossp";