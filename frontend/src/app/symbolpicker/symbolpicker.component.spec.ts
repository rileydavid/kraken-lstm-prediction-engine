import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SymbolpickerComponent } from './symbolpicker.component';

describe('SymbolpickerComponent', () => {
  let component: SymbolpickerComponent;
  let fixture: ComponentFixture<SymbolpickerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SymbolpickerComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(SymbolpickerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
