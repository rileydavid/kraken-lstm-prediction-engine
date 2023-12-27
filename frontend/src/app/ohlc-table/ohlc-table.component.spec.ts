import { ComponentFixture, TestBed } from '@angular/core/testing';

import { OhlcTableComponent } from './ohlc-table.component';

describe('OhlcTableComponent', () => {
  let component: OhlcTableComponent;
  let fixture: ComponentFixture<OhlcTableComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [OhlcTableComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(OhlcTableComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
