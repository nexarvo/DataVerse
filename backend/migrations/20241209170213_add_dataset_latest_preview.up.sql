-- Add latest_preview column to the dataset table
ALTER TABLE datasets
ADD COLUMN latest_preview jsonb DEFAULT '{}'::jsonb;