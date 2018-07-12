import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { TemperaturesTableComponent } from './temperatures-table.component';

describe('TemperaturesTableComponent', () => {
  let component: TemperaturesTableComponent;
  let fixture: ComponentFixture<TemperaturesTableComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ TemperaturesTableComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(TemperaturesTableComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
