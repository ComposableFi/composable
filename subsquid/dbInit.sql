-- Get the users with activity in the 24hs prior to a given hour.
CREATE OR REPLACE FUNCTION hourly_active_users (
  hours_ago INT
)
RETURNS bigint
LANGUAGE SQL
IMMUTABLE
AS $$
    SELECT
           count(distinct account_id)
    FROM activity
    WHERE
          timestamp < date_trunc('hour', current_timestamp) - $1 * interval '1 hour'
    AND
          timestamp > date_trunc('hour', current_timestamp) - $1 * interval '1 hour' - interval '1 day'
$$;

-- Get the users with activity in the `days_range` days prior to a given day.
CREATE OR REPLACE FUNCTION daily_active_users (
  days_ago INT,
  days_range INT
)
RETURNS bigint
LANGUAGE SQL
IMMUTABLE
AS $$
    SELECT
           count(distinct account_id)
    FROM activity
    WHERE
          timestamp < date_trunc('day', current_timestamp) - $1 * $2 * interval '1 day'
    AND
          timestamp > date_trunc('day', current_timestamp) - $1 * $2 * interval '1 day' - $2 * interval '1 day'
$$;

-- Get the latest total value locked previous to a given hour
CREATE OR REPLACE FUNCTION hourly_total_value_locked (
  hours_ago INT,
  source VARCHAR(30)
)
RETURNS bigint
LANGUAGE SQL
IMMUTABLE
AS $$
    SELECT
        COALESCE(amount, 0)
    FROM historical_locked_value
    WHERE
        timestamp < date_trunc('hour', current_timestamp) - $1 * interval '1 hour'
    AND historical_locked_value.source = $2
    ORDER BY timestamp DESC
    LIMIT 1
$$;

-- Get the latest total volume previous to a given hour
CREATE OR REPLACE FUNCTION hourly_total_volume (
  hours_ago INT
)
RETURNS bigint
LANGUAGE SQL
IMMUTABLE
AS $$
    SELECT
        COALESCE(amount, 0)
    FROM historical_volume
    WHERE
        timestamp < date_trunc('hour', current_timestamp) - $1 * interval '1 hour'
    ORDER BY timestamp DESC
    LIMIT 1
$$;