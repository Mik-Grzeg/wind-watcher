import React, { useEffect, useState } from 'react';
import Chart from 'chart.js/auto';

const mockData = [
  { label: 'Item 1', value: 10 },
  { label: 'Item 2', value: 20 },
  { label: 'Item 3', value: 30 },
  { label: 'Item 4', value: 40 },
  { label: 'Item 5', value: 50 }
];

const BarChart = () => {
  const [data, setData] = useState([]);

  useEffect(() => {
    const fetchData = async () => {
      // const response = await fetch('https://example.com/api/data');
      // const json = await response.json();
      setData(mockData);
    };

    fetchData();
  }, []);

  useEffect(() => {
    const canvas = document.getElementById('myChart');
    const ctx = canvas.getContext('2d');
    let chart = null;

    if (chart !== null) {
      chart.destroy();
    }

    chart = new Chart(ctx, {
      type: 'bar',
      data: {
        labels: data.map(datum => datum.label),
        datasets: [{
          label: 'My Bar Chart',
          data: data.map(datum => datum.value),
          backgroundColor: 'rgba(255, 99, 132, 0.2)',
          borderColor: 'rgba(255, 99, 132, 1)',
          borderWidth: 1
        }]
      },
      options: {
        scales: {
          y: {
            beginAtZero: true
          }
        }
      }
    });

    return () => chart.destroy();
  }, [data]);

  return (
    <div>
      <canvas id="myChart" width="400" height="400"></canvas>
    </div>
  );
};

export default BarChart;
