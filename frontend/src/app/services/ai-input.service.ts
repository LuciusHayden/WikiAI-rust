import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';
import { AiResponse, AiInput, Reference, References } from '../interfaces/wiki-query';
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

  getMainReference(): Observable<Reference> {
    return this.http.get<Reference>('http://localhost:3000/get-main-reference', {headers: this.headers});
  }

  getReferences(): Observable<References>{
    return this.http.get<References>('http://localhost:3000/get-references', {headers: this.headers});
  }

  setReferences(reference : Reference) {
    return this.http.post<Reference>('http://localhost:3000/set-references', reference, {headers: this.headers});
  }
}


