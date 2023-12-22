import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, map } from 'rxjs';
import { Ohlc } from '../models/ohlc.model';
import { Symbol } from '../models/symbol.model';

@Injectable({
  providedIn: 'root'
})
export class DataService {
  private baseUrl = 'http://localhost:8000'; 
  private url = 'http://localhost:8000/ohlc/hour/1/5'; 

  constructor(private http: HttpClient) { }

  fetchData(): Observable<any> {
    return this.http.get<any>(this.url);
  }

  fetchOhlcDayData() {
    return this.http.get<Ohlc[]>(this.baseUrl + "/ohlc/day/1/100").pipe(
      map(data => data.map(item => ({
        bucket: new Date(item.bucket),
        open_price: parseFloat(item.open_price.toString()),
        high: parseFloat(item.high.toString()),
        low: parseFloat(item.low.toString()),
        close_price: parseFloat(item.close_price.toString()),
        volume: parseFloat(item.volume.toString()),
        count: item.count,
        symbol: item.symbol
      })))     
    );
  }

  fetchOhlcHourData() {
    return this.http.get<Ohlc[]>(this.url).pipe(
      map(data => data.map(item => ({
        bucket: new Date(item.bucket),
        open_price: parseFloat(item.open_price.toString()),
        high: parseFloat(item.high.toString()),
        low: parseFloat(item.low.toString()),
        close_price: parseFloat(item.close_price.toString()),
        volume: parseFloat(item.volume.toString()),
        count: item.count,
        symbol: item.symbol
      })))     
    );
  }

  fetchSymbols(){ 
    return this.http.get<Symbol[]>(this.baseUrl + "/symbols").pipe(
      map(data => data.map(item => ({
        id: item.id,
        symbol: item.symbol
      })))     
    );
  }
}