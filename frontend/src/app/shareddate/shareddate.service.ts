import { Injectable } from '@angular/core';

const today = new Date();
const month = today.getMonth();
const year = today.getFullYear();

@Injectable({
  providedIn: 'root'
})
export class SharedDateService {
  private endDateTime: Date = today;
  private startDateTime: Date = new Date(today.getFullYear(), today.getMonth(), today.getDate(), today.getHours() - 1, today.getMinutes());

  constructor() {}

  setDates(startDate: Date, endDate: Date): void {
    console.log("setDates");
    this.startDateTime = startDate;
    this.endDateTime = endDate;
  }

  getStartDate(): Date {
    return this.startDateTime;
  }

  getEndDate(): Date {
    return this.startDateTime;
  }
}
