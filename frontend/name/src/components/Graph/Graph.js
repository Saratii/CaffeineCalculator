import React, { Component } from 'react';
import CanvasJSReact from '@canvasjs/react-charts';

const CanvasJSChart = CanvasJSReact.CanvasJSChart;

class Graph extends Component {
	render() {
		const options = {
			theme: "dark2",
			animationEnabled: true,
			zoomEnabled: true,
			title:{
				text: "Placeholder"
			},
			axisX: {
				title:"X",
				crosshair: {
					enabled: false,
					snapToDataPoint: true
				},
                minimum: -10,
                maximum: 10,
			},
			axisY:{
				title: "Y",
				crosshair: {
					enabled: true,
					snapToDataPoint: true
				},
                minimum: -10,
                maximum: 10,
                gridThickness: 0,
			},
			data: [{
				type: "scatter",
				markerSize: 1,
				dataPoints: this.props.dataPoints
			}]
		};

		return (
			<div>
				<CanvasJSChart options={options} />
			</div>
		);
	}
}

export default Graph;
