import { Component } from '@angular/core';
import { AiComponent, AiResponse } from '../../../interfaces/wiki-query';
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

  component : AiComponent = {
      input : { query :  "" },
      response : { response: "" },
      references : { references : []},
      main_reference : { link : ""},
    }

  constructor(private _aiInputService: AiInputService) {
   }

  processWikiSite() : void {
      this._aiInputService.processWikiSite(this.component.input).subscribe((response : AiResponse) => {
        this.component.response = response;
      })
  }

}
