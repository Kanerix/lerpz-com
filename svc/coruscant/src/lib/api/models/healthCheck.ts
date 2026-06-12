// @ts-nocheck

export interface HealthCheck {
    /** Whether the database connection is healthy */
    database: boolean;
    /** Whether the Redis connection is healthy */
    redis: boolean;
}
