import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Temperature } from './temperature';
import { Observable, config } from 'rxjs';
import { ConfigLoaderService } from './config-loader.service';
import { mergeMap } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class TemperaturesService {

  constructor(private configLoaderService: ConfigLoaderService, private httpClient: HttpClient) { }

  public getTemperatures(): Observable<Temperature[]> {
    return this.configLoaderService.getConfig().pipe(mergeMap((conf) => {
      return this.httpClient.get<Temperature[]>(conf.backend_base_url + "/api/temperatures");
    }));
  }
}
