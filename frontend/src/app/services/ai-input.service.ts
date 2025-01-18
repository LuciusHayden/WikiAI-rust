import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';
import { AiResponse, AiInput } from '../interfaces/wiki-query';
@Injectable({
  providedIn: 'root'
})
export class AiInputService {

  readonly headers : HttpHeaders;

  constructor(private http: HttpClient) {
    this.headers = new HttpHeaders({
      'Content-Type': 'application/json'
    });
  }

  processWikiSite(aiInput: AiInput): Observable<AiResponse> {
    return this.http.post<AiResponse>('http://localhost:3000/query', aiInput , {headers: this.headers});
  }
}

