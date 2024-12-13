-- Add parent_transformation_id column to the transformation table
ALTER TABLE transformations
ADD COLUMN parent_transformation_id UUID REFERENCES transformations(id) ON DELETE CASCADE;
