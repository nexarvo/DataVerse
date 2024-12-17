CREATE TABLE cell (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    name TEXT,
    first_transformation_id UUID REFERENCES transformations(id) ON DELETE SET NULL,
    input_dataframe_id UUID REFERENCES dataframe(id) ON DELETE SET NULL,
    input_dataset_id UUID REFERENCES datasets(id) ON DELETE SET NULL,
    result_dataframe_id UUID REFERENCES dataframe(id) ON DELETE SET NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    updated_at TIMESTAMP DEFAULT NOW(),
    updated_by UUID REFERENCES users(id) ON DELETE SET NULL
);