CREATE TABLE transformations (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    dataset_id UUID REFERENCES datasets(id) ON DELETE CASCADE,
    transformation_type TEXT NOT NULL, -- e.g., "filter", "aggregation"
    parameters JSONB NOT NULL,
    applied_at TIMESTAMP DEFAULT NOW(),
    applied_by UUID,
    result_preview JSONB
);