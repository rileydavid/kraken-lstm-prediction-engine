import { Component, EventEmitter, Output } from '@angular/core';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatFormFieldModule} from '@angular/material/form-field';
import { NgxMatDatetimePickerModule, NgxMatNativeDateModule, NgxMatTimepickerModule } from '@angular-material-components/datetime-picker';
import { MatInputModule } from '@angular/material/input';

// for testing purposes
const today: Date = new Date("2023-09-28T12:00:00");

@Component({
  selector: 'app-datepicker',
  standalone: true,
  imports: [
    NgxMatDatetimePickerModule,
    NgxMatNativeDateModule,
    NgxMatTimepickerModule,
    MatFormFieldModule,
    MatInputModule,
    FormsModule,
    ReactiveFormsModule,
    MatDatepickerModule,
    MatFormFieldModule,
  ],
  providers: [],
  templateUrl: './datepicker.component.html',
  styleUrl: './datepicker.component.css'
})
export class DatepickerComponent {
  @Output() fromDateChange = new EventEmitter<Date>();
  @Output() toDateChange = new EventEmitter<Date>();

  fromDateTime: Date = new Date(today.getFullYear(), today.getMonth(), today.getDate(), today.getHours(), 0);
  toDateTime: Date = new Date(today.getFullYear(), today.getMonth(), today.getDate(), today.getHours() + 12, 0);

  constructor() {}

  onDateChange(fromDateTime: Date, toDateTime: Date): void {
    this.setFromDate(fromDateTime);
    this.setToDate(toDateTime); 
  }

  ngOnInit(): void {
    this.setFromDate(this.fromDateTime);
    this.setToDate(this.toDateTime);
  }

  setFromDate(date: Date): void {
    this.fromDateChange.emit(date);
  }

  setToDate(date: Date): void {
    this.toDateChange.emit(date);
  }
}



