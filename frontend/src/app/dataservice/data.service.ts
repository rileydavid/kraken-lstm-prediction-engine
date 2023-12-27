import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { map } from 'rxjs';
import { Ohlc } from '../models/ohlc.model';
import { SymbolModel } from '../models/symbol.model';

const baseUrl = 'http://127.0.0.1:8000';
const ohlcHourRange = `${baseUrl}/ohlc/hour/range`;
const ohclHourPrediction = `${baseUrl}/execute/model`;

@Injectable({
  providedIn: 'root'
})
export class DataService {

  constructor(private http: HttpClient) { }

  fetchOhlcHourRangeData(fromDate: Date, toDate: Date, symbol_id: number) {
    console.log("fetchOhlcHourRangeData");
    const request_body = {
      "from_date": fromDate,
      "to_date": toDate,
      "symbol_id": symbol_id
    }

    console.log(ohlcHourRange)

    return this.http.post<Ohlc[]>(ohlcHourRange, request_body).pipe(
      map(data => data.map(item => ({
        bucket: new Date(item.bucket),
        open_price: parseFloat(item.open_price?.toString() ?? '0'),
        high: parseFloat(item.high?.toString() ?? '0'),
        low: parseFloat(item.low?.toString() ?? '0'),
        close_price: parseFloat(item.close_price.toString()),
        volume: parseFloat(item.volume?.toString() ?? '0'),
        count: item.count
      })))
    );
  }

  fetchOhlcPrediction(symbol_id: number, fromDate: Date) {
    console.log("fetchOhlcPrediction");
    const request_body = {
      "symbol_id": symbol_id,
      "from_date": fromDate
    }

    return this.http.post<Ohlc>(ohclHourPrediction, request_body).pipe(
      map(data => ({
        bucket: new Date(data.bucket),
        close_price: parseFloat(data.close_price.toString()),
      }))
    );
  }

  fetchSymbols() {
    return this.http.get<SymbolModel[]>(baseUrl + "/symbols").pipe(
      map(data => data.map(item => ({
        id: parseInt(item.id.toString()),
        symbol: item.symbol
      })))
    );
  }
}