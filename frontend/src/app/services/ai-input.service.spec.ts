import { TestBed } from '@angular/core/testing';

import { AiInputService } from './ai-input.service';

describe('AiInputService', () => {
  let service: AiInputService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(AiInputService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
