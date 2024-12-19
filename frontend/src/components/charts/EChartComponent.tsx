import React, { useEffect, useRef } from 'react';
import * as echarts from 'echarts';

const EChartComponent = ({ chartData, chartType }) => {
  const chartRef = useRef(null);

  useEffect(() => {
    const chartInstance = echarts.init(chartRef.current);

    const option = {
      title: {
        text: `${chartType} Chart`,
      },
      tooltip: {
        trigger: 'axis',
      },
      legend: {
        data: [chartData?.label || 'Dataset'],
      },
      xAxis: {
        type: 'category',
        data: chartData?.labels || [],
      },
      yAxis: {
        type: 'value',
      },
      series: [
        {
          name: chartData?.label || 'Dataset',
          type: chartType?.toLowerCase().replace(' ', ''),
          data: chartData?.values || [],
        },
      ],
    };

    // Set the options for the chart
    chartInstance.setOption(option);

    // Cleanup chart instance on component unmount
    return () => {
      chartInstance.dispose();
    };
  }, [chartData, chartType]);

  return <div ref={chartRef} style={{ height: '400px', width: '100%' }} />;
};

export default EChartComponent;
