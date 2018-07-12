import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { TemperaturesTableComponent } from './temperatures-table/temperatures-table.component';
import { TemperaturesService } from './temperatures.service';
import { HttpClientModule } from '@angular/common/http';

@NgModule({
  declarations: [
    AppComponent,
    TemperaturesTableComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
  ],
  providers: [
    TemperaturesService
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
