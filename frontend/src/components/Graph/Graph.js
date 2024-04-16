import React, { Component } from 'react';
import CanvasJSReact from '@canvasjs/react-charts';

const CanvasJSChart = CanvasJSReact.CanvasJSChart;

class Graph extends Component {
   render() {
        let { funct, minX, maxX, minY, maxY, dataPoints } = this.props;

        const options = {
            theme: "dark2",
            height: 600,
            width: 950,
            animationEnabled: true,
            interactivityEnabled: false,
            zoomEnabled: true,
            backgroundColor: "rgba(80, 0, 0, 0.2)",
            title: {
                 text: String(funct),
                 fontColor: "red",
                 fontFamily: "Verdana, Geneva, sans-serif",
                 fontSize: 25,
            },
            axisX: {
                crosshair: {
                    enabled: true,
                    snapToDataPoint: true
                },
                lineColor: null, 
                minimum: minX,
                maximum: maxX,
                gridThickness: 0.1,
                labelFontColor: "red",
                interval: Math.floor(Math.abs(maxX/5)),
                stripLines: [{                
                     value: 0,
                     color: "rgba(200, 0, 0, 0.5)",
                     thickness: 1,
                     labelFontColor: "red",
                     label: "y"
                }]
            },
            axisY:{
                crosshair: {
                    enabled: true,
                    snapToDataPoint: true
                },
                minimum: minY,
                maximum: maxY,
                gridThickness: 0.1,
                labelFontColor: "red",
                interval: Math.floor(Math.abs(maxY/5)),
                stripLines: [{                
                     value: 0,
                     color: "rgba(200, 0, 0, 0.5)",
                     thickness: 1,
                     labelFontColor: "red",
                     label: "x"
                }]
            },
            data: [{
                type: "line",
                markerType: "none",
                lineThickness: 2,
                color: "rgba(200, 0, 0, 1)",
                dataPoints: dataPoints
            }]
         };

       return (
            <div style={{ display: 'flex', justifyContent: 'left' }}>
                <CanvasJSChart options={options} />
            </div>
       );
   }
}

export default Graph;
