import { Component, OnInit } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { NgxChartsModule } from '@swimlane/ngx-charts';
import { Temperature } from "../temperature";
import { TemperaturesService } from '../temperatures.service';
import { Observable, timer } from 'rxjs';
import * as shape from 'd3-shape';

@Component({
  selector: 'app-temperatures-chart',
  templateUrl: './temperatures-chart.component.html',
  styleUrls: ['./temperatures-chart.component.css']
})
export class TemperaturesChartComponent implements OnInit {

  curve = shape.curveLinear;

  temperaturesList: Temperature[];
  temperaturesListGroupedBySensorID: any[];
  constructor(private temperaturesService: TemperaturesService) { }

  ngOnInit() {
    this.initAutomaticDataRefresh();
  }

  initAutomaticDataRefresh() {
  timer(0, 1000).subscribe(() => {
      this.updateTemperatures();
    });
  }

  updateTemperatures() {
    this.temperaturesService.getTemperatures().subscribe(items => {
      this.temperaturesList = items;
      this.temperaturesListGroupedBySensorID = [];
      let lastSensorID: Number;
      let lastSensorGroup;
      this.temperaturesList.sort((val1, val2) => { 
        const date1 = new Date(val1.time_year, val1.time_month, val1.time_day, val1.time_hour, val1.time_minute, val1.time_second, 0);
        const date2 = new Date(val2.time_year, val2.time_month, val2.time_day, val2.time_hour, val2.time_minute, val2.time_second, 0);
        if (val1.sensor_id !== val2.sensor_id) {
          return val1.sensor_id - val2.sensor_id;
        } else {
          return (date1.getTime() - date2.getTime());
        }
      });
      this.temperaturesList.forEach((val) => {
        if (lastSensorID === undefined || lastSensorID !== val.sensor_id) {
          lastSensorID = val.sensor_id;
          lastSensorGroup = {
            name: val.sensor_id,
            series: []
          };
          this.temperaturesListGroupedBySensorID.push(lastSensorGroup);
        }
        lastSensorGroup.series.push({
          name: new Date(val.time_year, val.time_month, val.time_day, val.time_hour, val.time_minute, val.time_second, 0),
          value: val.temperature
        });
      });
    });
  }
}
