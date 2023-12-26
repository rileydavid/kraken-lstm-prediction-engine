import { Component, EventEmitter, Output } from '@angular/core';
import { FormControl, FormsModule, ReactiveFormsModule } from '@angular/forms';
import {MatNativeDateModule} from '@angular/material/core';
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatFormFieldModule} from '@angular/material/form-field';
import { NgxMatDatetimePickerModule, NgxMatNativeDateModule, NgxMatTimepickerModule } from '@angular-material-components/datetime-picker';
import { MatInputModule } from '@angular/material/input';
import { SharedDateService } from '../shareddate/shareddate.service';


const today = new Date();
const month = today.getMonth();
const year = today.getFullYear();

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
  providers: [SharedDateService],
  templateUrl: './datepicker.component.html',
  styleUrl: './datepicker.component.css'
})
export class DatepickerComponent {
  @Output() startDateChange = new EventEmitter<Date>();
  @Output() endDateChange = new EventEmitter<Date>();

  endDateTime: Date = today;
  startDateTime: Date = new Date(today.getFullYear(), today.getMonth(), today.getDate(), today.getHours() - 1, today.getMinutes());

  constructor(private sharedDateService: SharedDateService) {}

  onDateChange(startDateTime: Date, endDateTime: Date): void {
    console.log("onDateChange");
    console.log("on date ", startDateTime, endDateTime);
    
    this.setStartDate(startDateTime);
    this.setEndDate(endDateTime); 
  }

  ngOnInit(): void {
  }

  setStartDate(date: Date): void {
    this.startDateChange.emit(date);
  }

  setEndDate(date: Date): void {
    this.endDateChange.emit(date);
  }

  /*
  getStartDateTimeUTC(): number {
    return this.startDateTime.getTime();
  }

  getEndDateTimeUTC(): number {
    return this.endDateTime.getTime();
  }
  */


}



