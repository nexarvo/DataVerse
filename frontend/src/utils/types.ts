// Define the types (interfaces)
type DatasetPreview = {
  headers: string[];
  data: string[];
};

type Dataset = {
  id: string;
  file_name: string;
  file_type: string;
  row_count: number;
  latest_preview: DatasetPreview;
};

export default Dataset;
export type { DatasetPreview };
