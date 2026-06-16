// @ts-nocheck

export interface HealthCheck {
  /** Whether the database connection is healthy */
  database: boolean;
  /** Whether the Redis connection is healthy */
  redis: boolean;
  /** Whether the S3 connection is healthy */
  s3: boolean;
}
