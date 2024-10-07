import { Component } from '@angular/core';
import { AiInput, AiResponse } from '../../../interfaces/wiki-query';
import { AiInputService } from '../../../services/ai-input.service';
import { FormsModule } from '@angular/forms';
import { HttpClientModule } from '@angular/common/http';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-ai-input',
  standalone: true,
  imports: [FormsModule, HttpClientModule, CommonModule],
  templateUrl: './ai-input.component.html',
  styleUrl: './ai-input.component.css'
})
export class AiInputComponent {

  input : AiInput = {
    url: '',
    question: ''
  }

  response : AiResponse = {
    result: ' ',
    id: 0
  }

  constructor(private _aiInputService: AiInputService) {
   }

  processWikiSite() : void {
      this._aiInputService.processWikiSite(this.input).subscribe((response : AiResponse) => {
        this.response = response;
      })
  }

}
