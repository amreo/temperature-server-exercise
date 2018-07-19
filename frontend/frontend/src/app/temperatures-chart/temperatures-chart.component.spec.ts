import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { TemperaturesChartComponent } from './temperatures-chart.component';

describe('TemperaturesChartComponent', () => {
  let component: TemperaturesChartComponent;
  let fixture: ComponentFixture<TemperaturesChartComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ TemperaturesChartComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(TemperaturesChartComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
