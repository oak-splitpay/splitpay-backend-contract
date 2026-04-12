import { Module } from '@nestjs/common';
import { SplitService } from './split.service';
import { SplitController } from './split.controller';
import { InMemoryDatabaseService } from '../database/in-memory.service';

@Module({
  controllers: [SplitController],
  providers: [SplitService, InMemoryDatabaseService],
})
export class SplitModule {}
