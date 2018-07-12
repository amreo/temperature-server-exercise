import { TestBed, inject } from '@angular/core/testing';

import { TemperaturesService } from './temperatures.service';

describe('TemperaturesService', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [TemperaturesService]
    });
  });

  it('should be created', inject([TemperaturesService], (service: TemperaturesService) => {
    expect(service).toBeTruthy();
  }));
});
