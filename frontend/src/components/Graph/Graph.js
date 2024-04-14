import React, { Component } from 'react';
import CanvasJSReact from '@canvasjs/react-charts';


const CanvasJSChart = CanvasJSReact.CanvasJSChart;


class Graph extends Component {
   render() {
        let { funct, minX, maxX, minY, maxY, dataPoints } = this.props;
        if (maxX === 0) { 
            maxX = 10;
        }
        if (maxY === 0) { 
            maxY = 10; 
        }

        const options = {
           theme: "dark2",
           animationEnabled: true,
           interactivityEnabled: false,
           zoomEnabled: true,
           backgroundColor: "rgba(80, 0, 0, 0.2)",
           title: {
                text: String(funct),
                fontColor: "red",
                fontFamily: "Verdana, Geneva, sans-serif"
           },
           axisX: {
               crosshair: {
                   enabled: true,
                   snapToDataPoint: false
               },
               lineColor: null, 
               minimum: minX,
               maximum: maxX,
               gridThickness: 0,
               labelFontColor: "red",
               interval: Math.floor(Math.abs(maxX/5))
           },
           axisY:{
               crosshair: {
                   enabled: true,
                   snapToDataPoint: false
               },
               minimum: minY,
               maximum: maxY,
               gridThickness: 0,
               labelFontColor: "red",
               interval: Math.floor(Math.abs(maxY/5))
           },
           data: [{
               type: "line",
               markerType: "none",
               lineThickness: 2,
               color: "rgba(200, 0, 0, 0.5)",
               dataPoints: dataPoints
           }]
       };


       return (
            <div style={{ width: '500px', height: '300px', padding: '40px'}}>
               <CanvasJSChart options={options} />
            </div>
       );
   }
}


export default Graph;
