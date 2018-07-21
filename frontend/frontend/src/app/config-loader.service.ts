import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { Config } from './config';
import { of } from 'rxjs';
import { concatMap } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class ConfigLoaderService {

  config: Config;

  constructor(private httpClient: HttpClient) {

  }

  getConfig(): Observable<Config> {
    if (!this.config) {
      return this.httpClient.get<Config>("/assets/config.json")
        .pipe(concatMap((conf) => {
          this.config = conf;
          console.log("Received config");
          return of(conf);
        }));
    } else {
      return of(this.config);
    }

  }
}
