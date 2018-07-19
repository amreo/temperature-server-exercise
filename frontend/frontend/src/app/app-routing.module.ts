import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { TemperaturesTableComponent } from './temperatures-table/temperatures-table.component';
import { TemperaturesChartComponent } from './temperatures-chart/temperatures-chart.component';


const routes: Routes = [
  { path: 'temperatures-chart', component: TemperaturesChartComponent, data: { title: 'Char of collected temperatures' } },
  { path: 'temperatures-table', component: TemperaturesTableComponent, data: { title: 'List of collected temperatures' } },
  { path: '', redirectTo: '/temperatures-table', pathMatch: 'full' },
];

@NgModule({
  imports: [RouterModule.forRoot(routes, { enableTracing: true } )],
  exports: [RouterModule]
})
export class AppRoutingModule { }
