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
  page: number;
  page_size: number;
  total_rows: number;
};

export default Dataset;
export type { DatasetPreview };
