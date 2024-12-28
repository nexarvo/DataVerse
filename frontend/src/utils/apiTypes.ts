type ApplyTransformationApiType = {
  type: string;
  action: string;
  params: ApplyTransformationParam;
};

export type ApplyTransformationParam = {
  column: string;
  operation: string;
  value: string;
};

export type GetDataframeByIdParam = {
  dataframe_id: string;
  page: number;
  page_size: number;
};

export type CreateCellParam = {
  name: string;
  input_dataframe_id?: string;
  input_dataset_id?: string;
  cell_type: string;
  cell_order?: number;
};

export type AddCellToPositionParam = {
  cell: CreateCellParam;
  reference_cell_id: string;
  cell_order: number;
};

export type GenerateChartParam = {
  dataset_id: string;
  is_dataset: boolean;
  chart_type: string;
  x_column: string;
  y_column: string;
  aggregation: string;
};

export type QuerySQLInputs = {
  id: string;
  data_type: string;
};

export type QuerySQLParam = {
  cell_id: string;
  sql_query: string;
  inputs: QuerySQLInputs[];
};

export default ApplyTransformationApiType;
