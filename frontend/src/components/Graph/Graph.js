import React, { Component } from 'react';
import CanvasJSReact from '@canvasjs/react-charts';


const CanvasJSChart = CanvasJSReact.CanvasJSChart;


class Graph extends Component {
   render() {
       const options = {
           theme: "dark2",
           animationEnabled: true,
           zoomEnabled: true,
           axisX: {
               crosshair: {
                   enabled: false,
                   snapToDataPoint: true
               },
               minimum: 0,
               maximum: 10,
               gridThickness: 0,
           },
           axisY:{
               crosshair: {
                   enabled: true,
                   snapToDataPoint: true
               },
               minimum: 0,
               maximum: 10,
               gridThickness: 0,
           },
           data: [{
               type: "line",
               markerSize: 5,
               dataPoints: this.props.dataPoints
           }]
       };


       return (
           <div style={{ width: '500px', height: '300px' }}>
               <CanvasJSChart options={options} />
           </div>
       );
   }
}


export default Graph;
