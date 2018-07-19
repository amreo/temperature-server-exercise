import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { TemperaturesTableComponent } from './temperatures-table/temperatures-table.component';
import { TemperaturesService } from './temperatures.service';
import { HttpClientModule } from '@angular/common/http';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { NgxChartsModule } from '@swimlane/ngx-charts';
import { TemperaturesChartComponent } from './temperatures-chart/temperatures-chart.component';

@NgModule({
  declarations: [
    AppComponent,
    TemperaturesTableComponent,
    TemperaturesChartComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    NgxChartsModule,
    BrowserAnimationsModule
  ],
  providers: [
    TemperaturesService
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
