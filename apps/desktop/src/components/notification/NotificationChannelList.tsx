import { useState } from 'react';
import { Bell, Trash2, TestTube2, Plus, Check, X } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Switch } from '@/components/ui/switch';
import { toast } from 'sonner';
import { invoke } from '@tauri-apps/api/core';
import { NotificationChannelDialog } from './NotificationChannelDialog';

export interface NotificationChannelDto {
  id: string;
  channel_type: string;
  config: string;  // JSON string from Rust
  enabled: boolean;
  created_at: string;
}

interface NotificationChannelListProps {
  channels: NotificationChannelDto[];
  onUpdate: () => void;
}

export function NotificationChannelList({ channels, onUpdate }: NotificationChannelListProps) {
  const [showDialog, setShowDialog] = useState(false);
  const [editingChannel, setEditingChannel] = useState<NotificationChannelDto | null>(null);
  const [testingId, setTestingId] = useState<string | null>(null);

  const handleDelete = async (channelId: string) => {
    try {
      await invoke('delete_notification_channel', { channelId });
      toast.success('通知渠道已删除');
      onUpdate();
    } catch (err) {
      toast.error('删除失败', {
        description: String(err),
      });
    }
  };

  const handleToggle = async (channel: NotificationChannelDto) => {
    try {
      await invoke('update_notification_channel', {
        input: {
          channel_id: channel.id,
          enabled: !channel.enabled,
          config: null,
        },
      });
      toast.success(channel.enabled ? '已禁用' : '已启用');
      onUpdate();
    } catch (err) {
      toast.error('更新失败', {
        description: String(err),
      });
    }
  };

  const handleTest = async (channelId: string) => {
    setTestingId(channelId);
    try {
      const result = await invoke<{ success: boolean; message: string }>('test_notification_channel', {
        channelId,
      });

      if (result.success) {
        toast.success('测试成功', {
          description: result.message,
        });
      } else {
        toast.error('测试失败', {
          description: result.message,
        });
      }
    } catch (err) {
      toast.error('测试失败', {
        description: String(err),
      });
    } finally {
      setTestingId(null);
    }
  };

  const handleEdit = (channel: NotificationChannelDto) => {
    setEditingChannel(channel);
    setShowDialog(true);
  };

  const handleAddNew = () => {
    setEditingChannel(null);
    setShowDialog(true);
  };

  const handleDialogClose = (success: boolean) => {
    setShowDialog(false);
    setEditingChannel(null);
    if (success) {
      onUpdate();
    }
  };

  const getChannelTypeName = (type: string) => {
    switch (type) {
      case 'feishu':
        return '飞书';
      case 'dingtalk':
        return '钉钉';
      case 'email':
        return '邮件';
      default:
        return type;
    }
  };

  const getChannelTypeColor = (type: string) => {
    switch (type) {
      case 'feishu':
        return 'bg-blue-500';
      case 'dingtalk':
        return 'bg-cyan-500';
      case 'email':
        return 'bg-purple-500';
      default:
        return 'bg-gray-500';
    }
  };

  const parseConfig = (configStr: string) => {
    try {
      return JSON.parse(configStr);
    } catch {
      return {};
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold">通知渠道</h3>
          <p className="text-sm text-muted-foreground">
            配置签到成功/失败时的通知方式
          </p>
        </div>
        <Button onClick={handleAddNew} className="rounded-full">
          <Plus className="h-4 w-4 mr-2" />
          添加渠道
        </Button>
      </div>

      {channels.length === 0 ? (
        <Card className="rounded-2xl">
          <CardContent className="py-12 text-center">
            <Bell className="h-12 w-12 mx-auto mb-4 text-muted-foreground opacity-50" />
            <p className="text-muted-foreground mb-4">还没有配置通知渠道</p>
            <Button onClick={handleAddNew} variant="outline" className="rounded-full">
              <Plus className="h-4 w-4 mr-2" />
              添加第一个通知渠道
            </Button>
          </CardContent>
        </Card>
      ) : (
        <div className="grid gap-4">
          {channels.map((channel) => {
            const config = parseConfig(channel.config);
            return (
              <Card key={channel.id} className="rounded-2xl">
                <CardContent className="py-4">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-4 flex-1">
                      <div className={`w-10 h-10 rounded-full ${getChannelTypeColor(channel.channel_type)} flex items-center justify-center`}>
                        <Bell className="h-5 w-5 text-white" />
                      </div>

                      <div className="flex-1">
                        <div className="flex items-center gap-2">
                          <h4 className="font-semibold">
                            {getChannelTypeName(channel.channel_type)}
                          </h4>
                          {channel.enabled ? (
                            <Badge variant="default" className="rounded-full">
                              <Check className="h-3 w-3 mr-1" />
                              已启用
                            </Badge>
                          ) : (
                            <Badge variant="secondary" className="rounded-full">
                              <X className="h-3 w-3 mr-1" />
                              已禁用
                            </Badge>
                          )}
                        </div>

                        <div className="text-sm text-muted-foreground mt-1">
                          {channel.channel_type === 'feishu' && config.webhook_key && (
                            <span>Webhook Key: {config.webhook_key.substring(0, 20)}...</span>
                          )}
                          {channel.channel_type === 'dingtalk' && config.webhook_key && (
                            <span>Webhook Key: {config.webhook_key.substring(0, 20)}...</span>
                          )}
                          {channel.channel_type === 'email' && config.from && (
                            <span>发件人: {config.from}</span>
                          )}
                        </div>
                      </div>
                    </div>

                    <div className="flex items-center gap-2">
                      <Switch
                        checked={channel.enabled}
                        onCheckedChange={() => handleToggle(channel)}
                      />

                      <Button
                        variant="outline"
                        size="icon"
                        className="rounded-full"
                        onClick={() => handleTest(channel.id)}
                        disabled={testingId === channel.id}
                      >
                        <TestTube2 className="h-4 w-4" />
                      </Button>

                      <Button
                        variant="outline"
                        size="icon"
                        className="rounded-full"
                        onClick={() => handleEdit(channel)}
                      >
                        <Bell className="h-4 w-4" />
                      </Button>

                      <Button
                        variant="outline"
                        size="icon"
                        className="rounded-full text-destructive hover:text-destructive"
                        onClick={() => handleDelete(channel.id)}
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>
                  </div>
                </CardContent>
              </Card>
            );
          })}
        </div>
      )}

      <NotificationChannelDialog
        open={showDialog}
        onClose={handleDialogClose}
        channel={editingChannel}
      />
    </div>
  );
}
