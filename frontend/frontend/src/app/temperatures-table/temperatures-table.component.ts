import { Component, OnInit } from '@angular/core';
import { TemperaturesService } from '../temperatures.service';
import { Temperature } from '../temperature';
import { Observable, timer } from 'rxjs';

@Component({
  selector: 'app-temperatures-table',
  templateUrl: './temperatures-table.component.html',
  styleUrls: ['./temperatures-table.component.css']
})
export class TemperaturesTableComponent implements OnInit {

  temperaturesList: Temperature[];

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
    this.temperaturesService.getTemperatures().subscribe(items => this.temperaturesList = items);
  }
}
