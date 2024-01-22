import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatFormFieldModule} from '@angular/material/form-field';
import { NgxMatDatetimePickerModule, NgxMatNativeDateModule, NgxMatTimepickerModule } from '@angular-material-components/datetime-picker';
import { MatInputModule } from '@angular/material/input';

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
export class DatepickerComponent implements OnInit {
  @Input() startDate: Date = new Date("2023-09-28T12:00:00");; // default value
  @Output() fromDateChange = new EventEmitter<Date>();
  @Output() toDateChange = new EventEmitter<Date>();

  fromDateTime: Date = new Date();
  toDateTime: Date = new Date();

  constructor() {}

  ngOnInit(): void {
    this.fromDateTime = new Date(this.startDate.getFullYear(), this.startDate.getMonth(), this.startDate.getDate(), this.startDate.getHours(), 0);
    this.toDateTime = new Date(this.startDate.getFullYear(), this.startDate.getMonth(), this.startDate.getDate(), this.startDate.getHours() + 12, 0);
  
    this.setFromDate(this.fromDateTime);
    this.setToDate(this.toDateTime);
  }

  onDateChange(fromDateTime: Date, toDateTime: Date): void {
    this.setFromDate(fromDateTime);
    this.setToDate(toDateTime); 
  }

  setFromDate(date: Date): void {
    this.fromDateChange.emit(date);
  }

  setToDate(date: Date): void {
    this.toDateChange.emit(date);
  }
}



