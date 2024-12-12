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

export default ApplyTransformationApiType;
