import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { map } from 'rxjs';
import { Ohlc } from '../models/ohlc.model';
import { SymbolModel } from '../models/symbol.model';
import { Trade } from '../models/trade.model';
import { environment } from '../../environments/environments';

//const baseUrl = 'http://172.1.0.14:8000';
//const baseUrl = 'http://127.0.0.1:8000';
const baseUrl = environment.apiUrl;

const ohlcHourRangeUrl = `${baseUrl}/ohlc/hour/range`;
const ohclHourPredictionUrl = `${baseUrl}/execute/model`;
const tradesUrl = `${baseUrl}/trades`;

@Injectable({
  providedIn: 'root'
})
export class DataService {

  constructor(private http: HttpClient) { }

  fetchOhlcHourRangeData(fromDate: Date, toDate: Date, symbol_id: number) {
    const request_body = {
      "from_date": fromDate,
      "to_date": toDate,
      "symbol_id": symbol_id
    }

    return this.http.post<Ohlc[]>(ohlcHourRangeUrl, request_body).pipe(
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
    const request_body = {
      "symbol_id": symbol_id,
      "from_date": fromDate
    }

    return this.http.post<Ohlc>(ohclHourPredictionUrl, request_body).pipe(
      map(data => ({
        bucket: new Date(data.bucket),
        close_price: parseFloat(data.close_price.toString()),
      }))
    );
  }

  fetchTrades(fromDate: Date, toDate: Date, symbol_id: number) {
    console.log("fetchTrades");
    const request_body = {  
      "from_date": fromDate,
      "to_date": toDate,
      "symbol_id": symbol_id
    }

    return this.http.post<Trade[]>(tradesUrl, request_body).pipe(
      map(dataArray => dataArray.map(data => ({
        time: new Date(data.time),
        price: parseFloat(data.price.toString()),
        volume: parseFloat(data.volume.toString()),
        side: data.side ?? '',
        order_type: data.order_type ?? '',
        symbol_id: data.symbol_id
      })))
    );
  }

  fetchSymbols() {
    console.log("is prod? ", environment.production);
    console.log(baseUrl);

    return this.http.get<SymbolModel[]>(baseUrl + "/symbols").pipe(
      map(data => data.map(item => ({
        id: parseInt(item.id.toString()),
        symbol: item.symbol
      })))
    );
  }
}