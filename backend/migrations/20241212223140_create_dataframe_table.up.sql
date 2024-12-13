CREATE TABLE dataframe (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    transformation_id UUID NOT NULL REFERENCES transformations(id) ON DELETE SET NULL,
    dataframe_duckdb_reference TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    updated_at TIMESTAMP DEFAULT NOW(),
    updated_by UUID REFERENCES users(id) ON DELETE SET NULL
);