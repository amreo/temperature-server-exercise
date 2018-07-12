import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Temperature } from './temperature';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class TemperaturesService {

  constructor(private httpClient: HttpClient) { }

  public getTemperatures(): Observable<Temperature[]> {
    return this.httpClient.get<Temperature[]>("http://127.0.0.1:8000/api/temperatures");
  }
}
