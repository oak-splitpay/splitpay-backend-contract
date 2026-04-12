import { Controller, Post, Get, Body, Param, BadRequestException, HttpCode } from '@nestjs/common';
import { SplitService } from './split.service';
import { CreateSplitDto } from './dtos/create-split.dto';
import { RecordPaymentDto } from './dtos/payment.dto';

@Controller('api/splits')
export class SplitController {
  constructor(private readonly splitService: SplitService) {}

  @Post()
  @HttpCode(201)
  async createSplit(@Body() createSplitDto: CreateSplitDto) {
    if (!createSplitDto.title || !createSplitDto.totalAmount || !createSplitDto.participants) {
      throw new BadRequestException('Missing required fields');
    }
    return this.splitService.createSession(createSplitDto);
  }

  @Get(':id')
  async getSplit(@Param('id') id: string) {
    return this.splitService.getSession(id);
  }

  @Get(':id/payments')
  async getPayments(@Param('id') splitId: string) {
    return this.splitService.getPayments(splitId);
  }

  @Post(':id/payments')
  @HttpCode(201)
  async recordPayment(
    @Param('id') splitId: string,
    @Body() recordPaymentDto: RecordPaymentDto,
  ) {
    if (recordPaymentDto.splitId !== splitId) {
      throw new BadRequestException('Split ID mismatch');
    }
    return this.splitService.recordPayment(recordPaymentDto);
  }

  @Get(':id/status')
  async getSplitStatus(@Param('id') id: string) {
    return this.splitService.getSplitStatus(id);
  }
}
