import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { Observable, throwError } from 'rxjs';
import { catchError, retry } from 'rxjs/operators';

import { HttpHeaders } from '@angular/common/http';
import { Trade } from '../model/trade';


@Injectable()
export class DataService {
    private baseUrl = "http://172.1.0.14:8000/";    

    constructor(private http: HttpClient) {}

    getTrades(interval: string, symbol: string) {
        //return this.http.get<Trade>(this.baseUrl + "trades/" + interval + "/" + symbol);
        return this.http.get<Trade>(this.baseUrl + "cached_trades/"  + symbol +"/"+ interval);
    }

    getPrediction(interval: string, symbol: string) {
        //return this.http.get<Trade>(this.baseUrl + "trades/" + interval + "/" + symbol);
        return this.http.get<String>(this.baseUrl + "prediction/"  + symbol +"/"+ interval);
    }
}