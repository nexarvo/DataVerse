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

export default ApplyTransformationApiType;
